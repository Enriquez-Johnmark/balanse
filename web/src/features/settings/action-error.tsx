import { useMutation } from "@tanstack/react-query"

import { api, ApiError } from "@/api/client"
import { SaveError } from "@/components/save-error"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"

/** Why an Owner-only action failed. Some need a login from the last 10
 *  minutes (the API decides which); for those, it offers a fresh login. */
export function ActionError({ error }: { error: Error | null }) {
  const logOutToLogIn = useMutation({
    mutationFn: () => api("POST", "/api/logout"),
    onSettled: () => location.assign("/login"),
  })
  if (!error) return null
  if (!(error instanceof ApiError && error.status === 403)) return <SaveError error={error} />
  return (
    <Alert variant="warning">
      <AlertTitle>Log in again to do this</AlertTitle>
      <AlertDescription className="flex flex-col items-start gap-2">
        <span>{error.message}</span>
        <Button
          variant="outline"
          size="sm"
          onClick={() => logOutToLogIn.mutate()}
          disabled={logOutToLogIn.isPending}
        >
          Log in again
        </Button>
      </AlertDescription>
    </Alert>
  )
}
