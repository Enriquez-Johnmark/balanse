import { useQuery } from "@tanstack/react-query"
import { CopyIcon, MessageCircleIcon, SmartphoneIcon } from "lucide-react"
import { useState } from "react"

import { api } from "@/api/client"
import type { CopyTexts } from "@/api/generated/CopyTexts"
import { messageKeys } from "@/app/query-keys"
import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { toast } from "@/components/ui/toast"

/** H4: a message's text for Viber, Messenger, WhatsApp or SMS. Copying
 *  approves and sends nothing; the texts come from the API, which refuses
 *  what it wouldn't send and keeps links and phone numbers out of the SMS
 *  version. */
export function CopyMenu({ messageId }: { messageId: string }) {
  const [isOpen, setIsOpen] = useState(false)
  // Fetched when the menu opens, so a click can copy at once: browsers
  // only allow copying straight after the click.
  const texts = useQuery({
    queryKey: messageKeys.copy(messageId),
    queryFn: () => api<CopyTexts>("GET", `/api/messages/${encodeURIComponent(messageId)}/copy`),
    enabled: isOpen,
    staleTime: 0,
  })
  const copy = async (text: string, what: string) => {
    try {
      await navigator.clipboard.writeText(text)
      toast.add({ title: `Copied for ${what}`, type: "success" })
    } catch {
      toast.add({ title: "Couldn't copy", description: "Your browser didn't allow it." })
    }
  }
  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <DropdownMenuTrigger render={<Button variant="outline" />}>
        <CopyIcon data-icon="inline-start" />
        Copy and open
      </DropdownMenuTrigger>
      <DropdownMenuContent align="start" className="min-w-64">
        {texts.isError ? (
          <p role="alert" className="max-w-72 px-1.5 py-1 text-sm text-destructive">
            {texts.error.message}
          </p>
        ) : null}
        <DropdownMenuItem
          disabled={!texts.data}
          onClick={() => texts.data && copy(texts.data.text, "Viber or Messenger")}
        >
          <CopyIcon />
          Copy for Viber/Messenger
        </DropdownMenuItem>
        <DropdownMenuItem
          disabled={!texts.data?.whatsapp_url}
          onClick={() => {
            if (texts.data?.whatsapp_url) {
              window.open(texts.data.whatsapp_url, "_blank", "noopener,noreferrer")
            }
          }}
        >
          <MessageCircleIcon />
          {texts.data && !texts.data.whatsapp_url
            ? "Open WhatsApp (no mobile number)"
            : "Open WhatsApp"}
        </DropdownMenuItem>
        <DropdownMenuItem
          disabled={!texts.data}
          onClick={() => texts.data && copy(texts.data.sms, "SMS")}
        >
          <SmartphoneIcon />
          Copy for SMS
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}
