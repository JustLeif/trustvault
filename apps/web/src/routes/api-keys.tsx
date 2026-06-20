import { createFileRoute } from "@tanstack/react-router"

import { PlaceholderRoute } from "@/components/placeholder-route"

export const Route = createFileRoute("/api-keys")({
  component: ApiKeysRoute,
})

function ApiKeysRoute() {
  return (
    <PlaceholderRoute
      title="API keys"
      description="Issue and rotate programmatic wallet access."
    />
  )
}
