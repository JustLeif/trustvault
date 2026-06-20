import { createFileRoute } from "@tanstack/react-router"

import { PlaceholderRoute } from "@/components/placeholder-route"

export const Route = createFileRoute("/settings")({
  component: SettingsRoute,
})

function SettingsRoute() {
  return (
    <PlaceholderRoute
      title="Settings"
      description="Configure policies, approvers, and security controls."
    />
  )
}
