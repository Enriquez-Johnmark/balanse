/** A message's text exactly as it will be sent, line breaks kept. */
export function MessageText({ subject, body }: { subject: string; body: string }) {
  return (
    <div className="flex flex-col gap-2 rounded-lg border bg-muted/50 p-3">
      <p className="font-medium">{subject}</p>
      <p className="text-sm break-words whitespace-pre-wrap">{body}</p>
    </div>
  )
}
