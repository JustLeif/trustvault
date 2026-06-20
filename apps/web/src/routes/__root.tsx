import { createRootRoute, Outlet } from "@tanstack/react-router"
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools"

import { AppMenu } from "@/components/app-menu"

const RootLayout = () => (
  <>
    <AppMenu>
      <Outlet />
    </AppMenu>
    <TanStackRouterDevtools />
  </>
)

export const Route = createRootRoute({ component: RootLayout })
