import { FormPage, FormState } from "@/components";

export default function TestingPage() {
  const state = new FormState(["a", "b"]);

  return (
    <div className="h-full w-full">
      <FormPage state={state} />
    </div>
  )
}
