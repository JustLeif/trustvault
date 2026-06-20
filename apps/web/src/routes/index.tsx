import {
  BadgeCheck,
  Bitcoin,
  Check,
  ChevronDown,
  CircleDollarSign,
  Clipboard,
  Code2,
  Copy,
  CreditCard,
  EyeOff,
  Fingerprint,
  KeyRound,
  MoreHorizontal,
  Plus,
  RefreshCcw,
  Search,
  ShieldCheck,
  Smartphone,
  TerminalSquare,
  Trash2,
  WalletCards,
} from "lucide-react"
import { useMemo, useState } from "react"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Separator } from "@/components/ui/separator"
import { cn } from "@/lib/utils"
import { createFileRoute } from "@tanstack/react-router"

export const Route = createFileRoute("/")({
  component: Index,
})

type Chain = "Bitcoin" | "Cardano"
type ApiKeyStatus = "Active" | "Rotating" | "Revoked"

type WalletKey = {
  id: string
  label: string
  chain: Chain
  fingerprint: string
  vault: string
  balance: string
  policy: string
  status: "Ready" | "Needs review"
  lastUsed: string
}

type ApiKey = {
  id: string
  name: string
  token: string
  scopes: string[]
  wallets: string[]
  created: string
  lastUsed: string
  status: ApiKeyStatus
}

const walletKeys: WalletKey[] = [
  {
    id: "btc-treasury",
    label: "Treasury cold key",
    chain: "Bitcoin",
    fingerprint: "xpub...8f3a",
    vault: "MPC vault A",
    balance: "12.489 BTC",
    policy: "3 of 5 approvals",
    status: "Ready",
    lastUsed: "18 minutes ago",
  },
  {
    id: "ada-staking",
    label: "Cardano staking key",
    chain: "Cardano",
    fingerprint: "addr...42be",
    vault: "Hardware enclave",
    balance: "884,200 ADA",
    policy: "2 of 3 approvals",
    status: "Ready",
    lastUsed: "2 hours ago",
  },
  {
    id: "btc-ops",
    label: "Operations hot key",
    chain: "Bitcoin",
    fingerprint: "bc1q...9d31",
    vault: "Policy vault",
    balance: "1.204 BTC",
    policy: "Daily cap: 0.25 BTC",
    status: "Needs review",
    lastUsed: "Yesterday",
  },
  {
    id: "ada-payments",
    label: "Cardano payments",
    chain: "Cardano",
    fingerprint: "stake...c80d",
    vault: "MPC vault B",
    balance: "72,450 ADA",
    policy: "Spend allowlist",
    status: "Ready",
    lastUsed: "Jun 18",
  },
]

const initialApiKeys: ApiKey[] = [
  {
    id: "api-prod",
    name: "Production settlement service",
    token: "tv_live_2nQm...k91Z",
    scopes: ["wallets:read", "transactions:create", "balances:read"],
    wallets: ["Treasury cold key", "Cardano payments"],
    created: "Jun 12",
    lastUsed: "4 minutes ago",
    status: "Active",
  },
  {
    id: "api-risk",
    name: "Risk monitoring",
    token: "tv_live_Y7c4...a20B",
    scopes: ["wallets:read", "balances:read"],
    wallets: ["All wallets"],
    created: "May 31",
    lastUsed: "21 minutes ago",
    status: "Active",
  },
  {
    id: "api-rotate",
    name: "Legacy payout worker",
    token: "tv_live_r9Bq...V4p8",
    scopes: ["transactions:create"],
    wallets: ["Operations hot key"],
    created: "Apr 9",
    lastUsed: "3 days ago",
    status: "Rotating",
  },
]

const chainStyles: Record<
  Chain,
  {
    icon: typeof Bitcoin
    className: string
  }
> = {
  Bitcoin: {
    icon: Bitcoin,
    className:
      "border-amber-200 bg-amber-50 text-amber-800 dark:border-amber-900/60 dark:bg-amber-950/40 dark:text-amber-300",
  },
  Cardano: {
    icon: CircleDollarSign,
    className:
      "border-sky-200 bg-sky-50 text-sky-800 dark:border-sky-900/60 dark:bg-sky-950/40 dark:text-sky-300",
  },
}

const statCards = [
  {
    label: "Total assets",
    value: "$1.82M",
    detail: "+4.8% from last week",
    icon: WalletCards,
  },
  {
    label: "Managed keys",
    value: "4",
    detail: "2 Bitcoin, 2 Cardano",
    icon: KeyRound,
  },
  {
    label: "Active API keys",
    value: "3",
    detail: "1 rotation pending",
    icon: Code2,
  },
  {
    label: "Policy coverage",
    value: "100%",
    detail: "All keys protected",
    icon: ShieldCheck,
  },
]

function Index() {
  const [apiKeys, setApiKeys] = useState(initialApiKeys)
  const [selectedWallet, setSelectedWallet] = useState("All wallets")
  const [selectedScopes, setSelectedScopes] = useState([
    "wallets:read",
    "balances:read",
  ])
  const [generatedKey, setGeneratedKey] = useState("")
  const [copiedValue, setCopiedValue] = useState("")

  const visibleWallets = useMemo(() => {
    if (selectedWallet === "All wallets") {
      return walletKeys
    }

    return walletKeys.filter((wallet) => wallet.label === selectedWallet)
  }, [selectedWallet])

  function toggleScope(scope: string) {
    setSelectedScopes((current) =>
      current.includes(scope)
        ? current.filter((item) => item !== scope)
        : [...current, scope]
    )
  }

  function createApiKey() {
    const token = `tv_live_${crypto.randomUUID().replaceAll("-", "").slice(0, 24)}`
    const keyName =
      selectedWallet === "All wallets"
        ? "Programmatic access key"
        : `${selectedWallet} API access`

    setGeneratedKey(token)
    setApiKeys((current) => [
      {
        id: crypto.randomUUID(),
        name: keyName,
        token: `${token.slice(0, 12)}...${token.slice(-4)}`,
        scopes: selectedScopes,
        wallets: [selectedWallet],
        created: "Just now",
        lastUsed: "Never",
        status: "Active",
      },
      ...current,
    ])
  }

  async function copyValue(value: string) {
    await navigator.clipboard?.writeText(value)
    setCopiedValue(value)
    window.setTimeout(() => setCopiedValue(""), 1200)
  }

  return (
    <>
      <header className="sticky top-0 z-20 border-b border-border bg-background/95 backdrop-blur">
        <div className="flex h-16 items-center gap-3 px-4 sm:px-6">
          <div className="flex size-9 items-center justify-center rounded-lg border border-border bg-card lg:hidden">
            <Fingerprint className="size-4" />
          </div>
          <div className="min-w-0 flex-1">
            <h1 className="truncate text-base font-semibold sm:text-lg">
              Private key dashboard
            </h1>
            <p className="hidden text-sm text-muted-foreground sm:block">
              Manage Bitcoin and Cardano custody access.
            </p>
          </div>
          <div className="hidden max-w-sm min-w-64 flex-1 items-center gap-2 rounded-lg border border-input bg-background px-3 shadow-xs md:flex">
            <Search className="size-4 text-muted-foreground" />
            <Input
              className="h-8 border-0 px-0 shadow-none focus-visible:ring-0"
              placeholder="Search keys, wallets, API tokens"
            />
          </div>
          <Button variant="outline" size="icon" aria-label="Refresh">
            <RefreshCcw />
          </Button>
          <Button>
            <Plus data-icon="inline-start" />
            New key
          </Button>
        </div>
      </header>

      <div className="mx-auto flex w-full max-w-7xl flex-col gap-5 px-4 py-5 sm:px-6 lg:gap-6 lg:py-6">
        <section className="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
          {statCards.map((stat) => {
            const Icon = stat.icon

            return (
              <Card key={stat.label}>
                <CardContent className="flex items-center justify-between gap-3 pt-4 sm:pt-5">
                  <div className="min-w-0">
                    <p className="text-sm text-muted-foreground">
                      {stat.label}
                    </p>
                    <div className="mt-1 text-2xl font-semibold tracking-normal">
                      {stat.value}
                    </div>
                    <p className="mt-1 text-xs text-muted-foreground">
                      {stat.detail}
                    </p>
                  </div>
                  <div className="flex size-10 shrink-0 items-center justify-center rounded-lg border border-border bg-secondary">
                    <Icon className="size-5 text-muted-foreground" />
                  </div>
                </CardContent>
              </Card>
            )
          })}
        </section>

        <section className="grid gap-5 xl:grid-cols-[minmax(0,1.45fr)_minmax(22rem,0.85fr)]">
          <Card>
            <CardHeader className="gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <CardTitle>Private keys</CardTitle>
                <CardDescription>
                  Custody inventory for supported chains.
                </CardDescription>
              </div>
              <div className="flex gap-2">
                <Button variant="outline" size="sm">
                  <EyeOff data-icon="inline-start" />
                  Masked
                </Button>
                <Button size="sm">
                  <Plus data-icon="inline-start" />
                  Add key
                </Button>
              </div>
            </CardHeader>
            <CardContent>
              <div className="hidden overflow-hidden rounded-lg border border-border md:block">
                <table className="w-full table-fixed text-sm">
                  <thead className="bg-muted/60 text-left text-xs text-muted-foreground">
                    <tr>
                      <th className="w-[30%] px-4 py-3 font-medium">
                        Wallet key
                      </th>
                      <th className="w-[16%] px-4 py-3 font-medium">Chain</th>
                      <th className="w-[18%] px-4 py-3 font-medium">Balance</th>
                      <th className="w-[20%] px-4 py-3 font-medium">Policy</th>
                      <th className="w-[16%] px-4 py-3 text-right font-medium">
                        Status
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {walletKeys.map((wallet) => (
                      <WalletRow key={wallet.id} wallet={wallet} />
                    ))}
                  </tbody>
                </table>
              </div>

              <div className="grid gap-3 md:hidden">
                {walletKeys.map((wallet) => (
                  <WalletMobileCard key={wallet.id} wallet={wallet} />
                ))}
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Generate API key</CardTitle>
              <CardDescription>
                Create scoped programmatic access for wallet automation.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <label htmlFor="wallet-scope" className="text-sm font-medium">
                  Wallet access
                </label>
                <div className="relative">
                  <select
                    id="wallet-scope"
                    value={selectedWallet}
                    onChange={(event) => setSelectedWallet(event.target.value)}
                    className="h-9 w-full appearance-none rounded-lg border border-input bg-background px-3 pr-9 text-sm shadow-xs outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
                  >
                    <option>All wallets</option>
                    {walletKeys.map((wallet) => (
                      <option key={wallet.id}>{wallet.label}</option>
                    ))}
                  </select>
                  <ChevronDown className="pointer-events-none absolute top-2.5 right-3 size-4 text-muted-foreground" />
                </div>
              </div>

              <div className="space-y-2">
                <div className="text-sm font-medium">Scopes</div>
                <div className="grid gap-2">
                  {[
                    "wallets:read",
                    "balances:read",
                    "transactions:create",
                    "transactions:sign",
                  ].map((scope) => {
                    const selected = selectedScopes.includes(scope)

                    return (
                      <button
                        key={scope}
                        type="button"
                        onClick={() => toggleScope(scope)}
                        className={cn(
                          "flex h-10 items-center justify-between rounded-lg border px-3 text-left text-sm transition-colors",
                          selected
                            ? "border-primary bg-primary text-primary-foreground"
                            : "border-border bg-background hover:bg-muted"
                        )}
                      >
                        <span>{scope}</span>
                        {selected ? <Check className="size-4" /> : null}
                      </button>
                    )
                  })}
                </div>
              </div>

              <div className="rounded-lg border border-border bg-muted/40 p-3">
                <div className="flex items-center gap-2 text-sm font-medium">
                  <TerminalSquare className="size-4" />
                  Access preview
                </div>
                <p className="mt-1 text-xs leading-5 text-muted-foreground">
                  {visibleWallets.length} wallet
                  {visibleWallets.length === 1 ? "" : "s"} with{" "}
                  {selectedScopes.length} scope
                  {selectedScopes.length === 1 ? "" : "s"} selected.
                </p>
              </div>

              <Button
                className="w-full"
                onClick={createApiKey}
                disabled={selectedScopes.length === 0}
              >
                <KeyRound data-icon="inline-start" />
                Generate API key
              </Button>

              {generatedKey ? (
                <div className="rounded-lg border border-emerald-200 bg-emerald-50 p-3 text-emerald-950 dark:border-emerald-900/60 dark:bg-emerald-950/40 dark:text-emerald-100">
                  <div className="flex items-center justify-between gap-3">
                    <div className="min-w-0">
                      <div className="text-sm font-medium">API key created</div>
                      <code className="mt-1 block truncate font-mono text-xs">
                        {generatedKey}
                      </code>
                    </div>
                    <Button
                      variant="outline"
                      size="icon"
                      aria-label="Copy generated API key"
                      onClick={() => copyValue(generatedKey)}
                    >
                      {copiedValue === generatedKey ? <Check /> : <Copy />}
                    </Button>
                  </div>
                </div>
              ) : null}
            </CardContent>
          </Card>
        </section>

        <section className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_22rem]">
          <Card>
            <CardHeader className="gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <CardTitle>Programmatic access</CardTitle>
                <CardDescription>
                  API keys, wallet scopes, and rotation state.
                </CardDescription>
              </div>
              <Button variant="outline" size="sm">
                <Clipboard data-icon="inline-start" />
                Audit export
              </Button>
            </CardHeader>
            <CardContent className="space-y-3">
              {apiKeys.map((apiKey) => (
                <div
                  key={apiKey.id}
                  className="grid gap-3 rounded-lg border border-border p-3 sm:grid-cols-[minmax(0,1fr)_auto] sm:items-center"
                >
                  <div className="min-w-0">
                    <div className="flex flex-wrap items-center gap-2">
                      <div className="font-medium">{apiKey.name}</div>
                      <StatusBadge status={apiKey.status} />
                    </div>
                    <div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-muted-foreground">
                      <code className="font-mono">{apiKey.token}</code>
                      <span>Created {apiKey.created}</span>
                      <span>Last used {apiKey.lastUsed}</span>
                    </div>
                    <div className="mt-3 flex flex-wrap gap-1.5">
                      {apiKey.scopes.map((scope) => (
                        <Badge key={scope} variant="outline">
                          {scope}
                        </Badge>
                      ))}
                    </div>
                  </div>
                  <div className="flex gap-2 sm:justify-end">
                    <Button
                      variant="outline"
                      size="icon"
                      aria-label={`Copy ${apiKey.name}`}
                      onClick={() => copyValue(apiKey.token)}
                    >
                      {copiedValue === apiKey.token ? <Check /> : <Copy />}
                    </Button>
                    <Button
                      variant="destructive"
                      size="icon"
                      aria-label={`Revoke ${apiKey.name}`}
                      onClick={() =>
                        setApiKeys((current) =>
                          current.map((item) =>
                            item.id === apiKey.id
                              ? { ...item, status: "Revoked" }
                              : item
                          )
                        )
                      }
                    >
                      <Trash2 />
                    </Button>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Security posture</CardTitle>
              <CardDescription>
                Controls protecting private key usage.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {[
                {
                  label: "Hardware-backed signing",
                  value: "Enabled",
                  icon: BadgeCheck,
                },
                {
                  label: "Spending limits",
                  value: "4 policies",
                  icon: CreditCard,
                },
                {
                  label: "Mobile approvals",
                  value: "5 approvers",
                  icon: Smartphone,
                },
              ].map((item) => {
                const Icon = item.icon

                return (
                  <div
                    key={item.label}
                    className="flex items-center justify-between gap-3"
                  >
                    <div className="flex min-w-0 items-center gap-3">
                      <div className="flex size-9 shrink-0 items-center justify-center rounded-lg border border-border bg-secondary">
                        <Icon className="size-4 text-muted-foreground" />
                      </div>
                      <div className="min-w-0">
                        <div className="truncate text-sm font-medium">
                          {item.label}
                        </div>
                        <div className="text-xs text-muted-foreground">
                          {item.value}
                        </div>
                      </div>
                    </div>
                    <Badge>On</Badge>
                  </div>
                )
              })}
              <Separator />
              <div>
                <div className="text-sm font-medium">Recent activity</div>
                <div className="mt-3 space-y-3">
                  {[
                    "API key used for BTC balance read",
                    "Cardano staking key approval completed",
                    "Legacy payout worker marked for rotation",
                  ].map((event) => (
                    <div
                      key={event}
                      className="flex items-start gap-2 text-sm text-muted-foreground"
                    >
                      <span className="mt-2 size-1.5 shrink-0 rounded-full bg-foreground" />
                      <span>{event}</span>
                    </div>
                  ))}
                </div>
              </div>
            </CardContent>
          </Card>
        </section>
      </div>
    </>
  )
}

function WalletRow({ wallet }: { wallet: WalletKey }) {
  const style = chainStyles[wallet.chain]
  const ChainIcon = style.icon

  return (
    <tr className="border-t border-border">
      <td className="px-4 py-3">
        <div className="min-w-0">
          <div className="truncate font-medium">{wallet.label}</div>
          <div className="mt-1 truncate font-mono text-xs text-muted-foreground">
            {wallet.fingerprint}
          </div>
        </div>
      </td>
      <td className="px-4 py-3">
        <Badge variant="outline" className={style.className}>
          <ChainIcon className="size-3" />
          {wallet.chain}
        </Badge>
      </td>
      <td className="px-4 py-3">
        <div className="font-medium">{wallet.balance}</div>
        <div className="text-xs text-muted-foreground">{wallet.vault}</div>
      </td>
      <td className="px-4 py-3 text-muted-foreground">{wallet.policy}</td>
      <td className="px-4 py-3 text-right">
        <div className="flex items-center justify-end gap-2">
          <Badge className="justify-center">{wallet.status}</Badge>
          <Button variant="ghost" size="icon-sm" aria-label="Wallet actions">
            <MoreHorizontal />
          </Button>
        </div>
      </td>
    </tr>
  )
}

function WalletMobileCard({ wallet }: { wallet: WalletKey }) {
  const style = chainStyles[wallet.chain]
  const ChainIcon = style.icon

  return (
    <div className="rounded-lg border border-border p-3">
      <div className="flex items-start justify-between gap-3">
        <div className="min-w-0">
          <div className="truncate font-medium">{wallet.label}</div>
          <div className="mt-1 truncate font-mono text-xs text-muted-foreground">
            {wallet.fingerprint}
          </div>
        </div>
        <Badge>{wallet.status}</Badge>
      </div>
      <div className="mt-3 grid grid-cols-2 gap-3 text-sm">
        <div>
          <div className="text-xs text-muted-foreground">Chain</div>
          <Badge variant="outline" className={cn("mt-1", style.className)}>
            <ChainIcon className="size-3" />
            {wallet.chain}
          </Badge>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Balance</div>
          <div className="mt-1 font-medium">{wallet.balance}</div>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Vault</div>
          <div className="mt-1">{wallet.vault}</div>
        </div>
        <div>
          <div className="text-xs text-muted-foreground">Policy</div>
          <div className="mt-1">{wallet.policy}</div>
        </div>
      </div>
    </div>
  )
}

function StatusBadge({ status }: { status: ApiKeyStatus }) {
  if (status === "Active") {
    return <Badge>Active</Badge>
  }

  if (status === "Rotating") {
    return <Badge variant="ghost">Rotating</Badge>
  }

  return <Badge variant="destructive">Revoked</Badge>
}
