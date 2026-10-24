import { zodResolver } from "@hookform/resolvers/zod"
import { useMutation } from "@tanstack/react-query"
import { MailCheckIcon } from "lucide-react"
import { Controller, useForm } from "react-hook-form"
import { Link } from "react-router"
import { z } from "zod"

import { api } from "@/api/client"
import type { SignupRequest } from "@/api/generated/SignupRequest"
import { CenteredLayout } from "@/components/centered-layout"
import { SaveError } from "@/components/save-error"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"

const schema = z.object({
  business_name: z
    .string()
    .trim()
    .min(1, "Enter your business's name.")
    .max(100, "Keep it to 100 characters."),
  email: z.email("Enter your email address."),
  // Starts unanswered, and must be answered.
  lends_money: z
    .string()
    .refine((answer) => answer === "yes" || answer === "no", "Tell us whether it does."),
  accepted_terms: z.boolean().refine((accepted) => accepted, "Accept the terms to continue."),
})

type SignupForm = z.input<typeof schema>
type Answered = z.output<typeof schema>

/** A3: asking for an account. A business that lends money is refused by
 *  the API with the reason (R16); other requests wait for the operator. */
export function SignupPage() {
  const form = useForm<SignupForm, unknown, Answered>({
    resolver: zodResolver(schema),
    defaultValues: { business_name: "", email: "", lends_money: "", accepted_terms: false },
  })
  const errors = form.formState.errors
  const request = useMutation({
    mutationFn: (body: SignupRequest) => api("POST", "/api/signup", body),
  })
  const submit = (values: Answered) =>
    request.mutate({ ...values, lends_money: values.lends_money === "yes" })

  return (
    <CenteredLayout>
      <Card>
        <CardHeader>
          <CardTitle>Request a Balanse account</CardTitle>
          <CardDescription>We'll email you a login link once your account is open.</CardDescription>
        </CardHeader>
        <CardContent>
          {request.isSuccess ? (
            <Alert>
              <MailCheckIcon />
              <AlertTitle>Thanks! Your request is in</AlertTitle>
              <AlertDescription>
                We'll email {form.getValues("email")} when the account for{" "}
                {form.getValues("business_name")} is ready, usually within 2 working days.
              </AlertDescription>
            </Alert>
          ) : (
            <form onSubmit={form.handleSubmit(submit)} noValidate>
              <FieldGroup>
                <Field data-invalid={errors.business_name ? true : undefined}>
                  <FieldLabel htmlFor="business-name">Business name</FieldLabel>
                  <Input
                    id="business-name"
                    autoComplete="organization"
                    aria-invalid={errors.business_name ? true : undefined}
                    {...form.register("business_name")}
                  />
                  <FieldDescription>Your reminders are signed with it.</FieldDescription>
                  <FieldError errors={[errors.business_name]} />
                </Field>
                <Field data-invalid={errors.email ? true : undefined}>
                  <FieldLabel htmlFor="email">Your email</FieldLabel>
                  <Input
                    id="email"
                    type="email"
                    autoComplete="email"
                    aria-invalid={errors.email ? true : undefined}
                    {...form.register("email")}
                  />
                  <FieldError errors={[errors.email]} />
                </Field>
                <FieldSet data-invalid={errors.lends_money ? true : undefined}>
                  <FieldLegend variant="label">
                    Does your business lend money, finance purchases or pawn items?
                  </FieldLegend>
                  <div className="flex gap-6">
                    {(["no", "yes"] as const).map((answer) => (
                      <Field key={answer} orientation="horizontal">
                        <input
                          id={`lends-${answer}`}
                          type="radio"
                          value={answer}
                          className="size-4 accent-primary"
                          {...form.register("lends_money")}
                        />
                        <FieldLabel htmlFor={`lends-${answer}`} className="font-normal">
                          {answer === "yes" ? "Yes" : "No"}
                        </FieldLabel>
                      </Field>
                    ))}
                  </div>
                  <FieldError errors={[errors.lends_money]} />
                </FieldSet>
                <Controller
                  control={form.control}
                  name="accepted_terms"
                  render={({ field }) => (
                    <Field
                      orientation="horizontal"
                      data-invalid={errors.accepted_terms ? true : undefined}
                    >
                      <Checkbox
                        id="accept"
                        checked={field.value}
                        onCheckedChange={field.onChange}
                        aria-invalid={errors.accepted_terms ? true : undefined}
                      />
                      <FieldLabel htmlFor="accept" className="font-normal">
                        I accept the terms and the data processing agreement.
                      </FieldLabel>
                    </Field>
                  )}
                />
                <FieldError errors={[errors.accepted_terms]} />
                {request.isError ? <SaveError error={request.error} /> : null}
                <Button type="submit" disabled={request.isPending}>
                  {request.isPending ? <Spinner data-icon="inline-start" /> : null}
                  Request an account
                </Button>
                <p className="text-center text-sm text-muted-foreground">
                  Already have one?{" "}
                  <Link to="/login" className="underline underline-offset-4">
                    Log in
                  </Link>
                </p>
              </FieldGroup>
            </form>
          )}
        </CardContent>
      </Card>
    </CenteredLayout>
  )
}
