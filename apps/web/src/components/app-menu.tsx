import {
  Activity,
  Braces,
  Fingerprint,
  LayoutDashboard,
  LockKeyhole,
  Settings,
  ShieldCheck,
} from "lucide-react"
import type { ReactNode } from "react"
import { Link } from "@tanstack/react-router"

import { Button } from "@/components/ui/button"

const navItems = [
  { label: "Dashboard", icon: LayoutDashboard, to: "/" },
  { label: "Private Keys", icon: LockKeyhole, to: "/private-keys" },
  { label: "API Keys", icon: Braces, to: "/api-keys" },
  { label: "Activity", icon: Activity, to: "/activity" },
  { label: "Settings", icon: Settings, to: "/settings" },
] as const

function AppMenu({ children }: { children: ReactNode }) {
  return (
    <div className="h-svh overflow-hidden bg-background text-foreground">
      <div className="grid h-svh lg:grid-cols-[16rem_minmax(0,1fr)]">
        <aside className="hidden h-svh border-r border-border bg-sidebar/70 lg:flex lg:flex-col">
          <div className="flex h-16 items-center gap-3 px-5">
            <div className="flex size-9 items-center justify-center rounded-lg border border-border bg-background">
              <Fingerprint className="size-4" />
            </div>
            <div className="min-w-0">
              <div className="text-sm font-semibold">TrustVault</div>
              <div className="text-xs text-muted-foreground">Key custody</div>
            </div>
          </div>
          <nav className="flex flex-1 flex-col gap-1 px-3 py-3">
            {navItems.map((item) => {
              const Icon = item.icon

              return (
                <Button
                  key={item.label}
                  asChild
                  variant="ghost"
                  className="justify-start [&.active]:bg-secondary [&.active]:text-secondary-foreground"
                >
                  <Link to={item.to}>
                    <Icon data-icon="inline-start" />
                    {item.label}
                  </Link>
                </Button>
              )
            })}
          </nav>
          <div className="p-4">
            <div className="rounded-lg border border-border bg-background p-3">
              <div className="flex items-center gap-2 text-sm font-medium">
                <ShieldCheck className="size-4 text-emerald-600" />
                Custody healthy
              </div>
              <p className="mt-1 text-xs leading-5 text-muted-foreground">
                Hardware checks and signing policies are current.
              </p>
            </div>
          </div>
        </aside>

        <main className="min-w-0 overflow-y-auto pb-20 lg:pb-0">
          {children}
        </main>
      </div>

      <nav className="fixed inset-x-0 bottom-0 z-30 grid grid-cols-5 border-t border-border bg-background/95 px-2 py-2 backdrop-blur lg:hidden">
        {navItems.map((item) => {
          const Icon = item.icon

          return (
            <Link
              key={item.label}
              to={item.to}
              className="flex h-12 flex-col items-center justify-center gap-1 rounded-lg text-[0.7rem] text-muted-foreground [&.active]:bg-secondary [&.active]:text-foreground"
            >
              <Icon className="size-4" />
              <span className="max-w-full truncate">{item.label}</span>
            </Link>
          )
        })}
      </nav>
    </div>
  )
}

export { AppMenu }
