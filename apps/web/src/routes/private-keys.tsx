import { createFileRoute } from "@tanstack/react-router"

import { PlaceholderRoute } from "@/components/placeholder-route"

export const Route = createFileRoute("/private-keys")({
  component: PrivateKeysRoute,
})

function PrivateKeysRoute() {
  return (
    <PlaceholderRoute
      title="Private keys"
      description="Manage Bitcoin and Cardano custody keys."
    />
  )
}
