"""Throwaway prototype: the blind Taglish test for Balanse, Claude Haiku 5.5 vs
Sonnet 5.5, as Kompleto ran it (Kompleto's docs/decisions.md, 2026-11-09).

Answers one question: is Haiku's Taglish good enough for Balanse's reminders,
or should Sonnet word them? Not product code. Delete once the decision is
recorded in docs/decisions.md.

    python3 -I prototypes/taglish-test/run.py draft   both models write every case
    python3 -I prototypes/taglish-test/run.py judge   a blind judge picks the better email

Both use the owner's key (OWNER_ANTHROPIC_API_KEY, or ANTHROPIC_API_KEY) and
Balanse's exact request: the system prompt read from src/balanse/drafting.rs,
the engine's request body, and the six facts the model may see (E1). Every
business, person and invoice is made up. Results go to out/ (ignored by git).
"""

import json
import os
import re
import sys
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
OUT = HERE / "out"
MODELS = {"haiku": "claude-haiku-5-5", "sonnet": "claude-sonnet-5-5"}
# The engine's thinking setting per model (nudgewright src/llm.rs `body`).
THINKING = {"haiku": "disabled", "sonnet": "between_tools"}
SCHEMA = {"type": "object",
          "properties": {"subject": {"type": "string"}, "opening": {"type": "string"}, "closing": {"type": "string"}},
          "required": ["subject", "opening", "closing"], "additionalProperties": False}
BUSINESS = "Reyes Creative Studio"
# (debtor, contact, open invoices, overdue range, step, invoice lines as Balanse lists them)
CASES = [
    ("Santos Trading Corp", "Lito", 1, "not_yet_due", "heads_up",
     ["- Invoice RCS-2026-041 (1 Nov 2026): ₱45,000.00, due 20 Nov 2026"]),
    ("Cruz & Co. Events", "Mara", 1, "up_to_a_week", "first",
     ["- Invoice RCS-2026-034 (10 Oct 2026): ₱32,000.00, due 9 Nov 2026"]),
    ("Garcia Dental Clinic", "Rina", 2, "up_to_a_month", "first",
     ["- Invoice RCS-2026-037 (18 Sep 2026): ₱12,000.00, due 18 Oct 2026",
      "- Invoice RCS-2026-044 (5 Oct 2026): ₱6,500.00, due 4 Nov 2026"]),
    ("Ocampo Bakeshop", "Joy", 1, "up_to_a_week", "second",
     ["- Invoice RCS-2026-040 (1 Oct 2026): ₱15,000.00, due 31 Oct 2026"]),
    ("Tan Prints", "Ben", 1, "up_to_a_month", "second",
     ["- Invoice RCS-2026-039 (25 Sep 2026): ₱9,800.00, due 25 Oct 2026"]),
    ("Lim Logistics", "Grace", 1, "up_to_three_months", "third",
     ["- Invoice RCS-2026-036 (15 Aug 2026): ₱60,000.00, due 15 Sep 2026"]),
    ("Bautista Media", "Jun", 3, "up_to_three_months", "third",
     ["- Invoice RCS-2026-030 (1 Aug 2026): ₱27,500.00, due 31 Aug 2026",
      "- Invoice RCS-2026-035 (12 Sep 2026): ₱8,000.00, due 12 Oct 2026",
      "- Invoice RCS-2026-042 (2 Oct 2026): ₱4,500.00, due 1 Nov 2026"]),
    ("Villanueva Tutorial Center", "Ana", 1, "not_yet_due", "heads_up",
     ["- Invoice RCS-2026-045 (5 Nov 2026): ₱18,000.00, due 25 Nov 2026"]),
    ("Dizon Auto Supply", "Rey", 1, "over_three_months", "third",
     ["- Invoice RCS-2026-021 (2 Jun 2026): ₱22,000.00, due 2 Jul 2026"]),
    ("Mendoza Catering", "Liza", 2, "up_to_a_week", "first",
     ["- Invoice RCS-2026-038 (20 Sep 2026): ₱35,000.00, due 3 Nov 2026",
      "- Invoice RCS-2026-043 (25 Oct 2026): ₱7,200.00, due 5 Nov 2026"]),
]
STAGE = {"heads_up": "a heads-up before the due date", "first": "the first reminder",
         "second": "the second reminder", "third": "the third and firmest reminder"}
JUDGE_MODEL = "claude-opus-5-5"
JUDGE = """You run a small design studio in Manila and chase client companies for unpaid invoices by email.
You will see two reminder emails to the same client contact. Pick the one you would rather send.
Judge: natural Taglish as Filipino businesses really write to clients (not stiff, not textbook Tagalog, not
English with a few Tagalog words added); politeness and respect ("po") without grovelling; a tone that fits
the reminder's stage (friendly early, firm but never threatening at the end); and clarity. The invoice list,
link and footer are identical in both; judge only the wording around them. Answer "tie" only if you truly
cannot choose."""
VERDICT = {"type": "object", "properties": {"better": {"type": "string", "enum": ["first", "second", "tie"]},
           "reason": {"type": "string"}}, "required": ["better", "reason"], "additionalProperties": False}


def key():
    found = os.environ.get("ANTHROPIC_API_KEY") or os.environ.get("OWNER_ANTHROPIC_API_KEY")
    return found or sys.exit("Set OWNER_ANTHROPIC_API_KEY (or ANTHROPIC_API_KEY) first.")


def app_system():
    """SYSTEM exactly as drafting.rs has it."""
    rust = (ROOT / "src/balanse/drafting.rs").read_text()
    literal = re.search(r'const SYSTEM: &str = "(.*?)";\n', rust, re.S).group(1)
    # A backslash at a line's end joins the next line, dropping its indent.
    return re.sub(r"\\\n\s*", "", literal).replace('\\"', '"')


def call(body):
    request = urllib.request.Request("https://api.anthropic.com/v1/messages", method="POST",
                                     data=json.dumps(body).encode(), headers={
                                         "x-api-key": key(), "anthropic-version": "2023-06-01",
                                         "content-type": "application/json"})
    with urllib.request.urlopen(request, timeout=180) as response:
        reply = json.load(response)
    text = next(block["text"] for block in reply["content"] if block["type"] == "text")
    return json.loads(text), reply


def brief(case):
    debtor, contact, invoices, overdue, step, _ = case
    # The six facts in the order Balanse's Brief serialises them (E1).
    return {"debtor_name": debtor, "contact_first_name": contact, "open_invoices": invoices,
            "overdue": overdue, "language": "taglish", "step": step}


def email(case, wording):
    """The reminder as Balanse assembles it (wording.rs `reminder`)."""
    _, contact, _, _, _, lines = case
    footer = (f"--\nSent via Balanse on behalf of {BUSINESS}. {BUSINESS} is responsible for your details in "
              "this email, and Balanse handles them on its behalf. To stop these emails, reply STOP.")
    body = (f"Hi {contact},\n\n{wording['opening'].strip()}\n\n" + "\n".join(lines) +
            "\n\nMakikita po dito ang mga detalye at kung paano magbayad: https://balanseapp.com/d/…\n\n"
            f"{wording['closing'].strip()}\n\n{BUSINESS}\n\n{footer}\n")
    return {"subject": wording["subject"].strip(), "body": body}


def draft():
    system, answers, usage = app_system(), {}, {}
    for number, case in enumerate(CASES, 1):
        for name, model in MODELS.items():
            wording, reply = call({"model": model, "max_tokens": 1000, "system": system,
                                   "messages": [{"role": "user", "content": json.dumps(brief(case), ensure_ascii=False)}],
                                   "output_config": {"format": {"type": "json_schema", "schema": SCHEMA}},
                                   "thinking": {"type": THINKING[name]}})
            answers[f"{number}-{name}"] = wording
            usage[f"{number}-{name}"] = {"model": reply["model"], **reply["usage"]}
            print(f"case {number} {name}: {reply['stop_reason']}")
    OUT.mkdir(exist_ok=True)
    (OUT / "answers.json").write_text(json.dumps(answers, ensure_ascii=False, indent=1))
    (OUT / "usage.json").write_text(json.dumps(usage, indent=1))


def ask_judge(case, first, second):
    text = (f"This is {STAGE[case[4]]} to {case[1]} of {case[0]}.\n\n"
            f"FIRST EMAIL\nSubject: {first['subject']}\n\n{first['body']}\n\n"
            f"SECOND EMAIL\nSubject: {second['subject']}\n\n{second['body']}")
    verdict, _ = call({"model": JUDGE_MODEL, "max_tokens": 1000, "system": JUDGE,
                       "messages": [{"role": "user", "content": text}],
                       "output_config": {"format": {"type": "json_schema", "schema": VERDICT}}})
    return verdict


def judge():
    answers = json.loads((OUT / "answers.json").read_text())
    results, totals = [], {"haiku": 0, "sonnet": 0, "tie": 0}
    for number, case in enumerate(CASES, 1):
        haiku, sonnet = (email(case, answers[f"{number}-{model}"]) for model in ("haiku", "sonnet"))
        # Asked twice, the order swapped: a verdict that flips with the order is a tie.
        one, two = ask_judge(case, haiku, sonnet), ask_judge(case, sonnet, haiku)
        picks = ({"first": "haiku", "second": "sonnet", "tie": "tie"}[one["better"]],
                 {"first": "sonnet", "second": "haiku", "tie": "tie"}[two["better"]])
        winner = picks[0] if picks[0] == picks[1] else "tie"
        totals[winner] += 1
        results.append({"case": number, "debtor": case[0], "winner": winner, "picks": picks,
                        "reasons": [one["reason"], two["reason"]]})
        print(f"case {number}: {winner}  ({picks[0]} / {picks[1]})")
    (OUT / "judged.json").write_text(json.dumps({"judge": JUDGE_MODEL, "totals": totals, "cases": results},
                                                ensure_ascii=False, indent=1))
    print("totals:", totals)


if __name__ == "__main__":
    {"draft": draft, "judge": judge}.get(sys.argv[1] if len(sys.argv) > 1 else "", lambda: sys.exit(__doc__))()
