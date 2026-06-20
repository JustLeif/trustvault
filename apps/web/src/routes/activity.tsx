import { createFileRoute } from "@tanstack/react-router"

import { PlaceholderRoute } from "@/components/placeholder-route"

export const Route = createFileRoute("/activity")({
  component: ActivityRoute,
})

function ActivityRoute() {
  return (
    <PlaceholderRoute
      title="Activity"
      description="Review signing, access, and approval events."
    />
  )
}
