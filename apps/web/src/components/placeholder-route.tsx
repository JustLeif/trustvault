import { Search, Plus } from "lucide-react"

import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"

function PlaceholderRoute({
  title,
  description,
}: {
  title: string
  description: string
}) {
  return (
    <>
      <header className="sticky top-0 z-20 border-b border-border bg-background/95 backdrop-blur">
        <div className="flex h-16 items-center gap-3 px-4 sm:px-6">
          <div className="min-w-0 flex-1">
            <h1 className="truncate text-base font-semibold sm:text-lg">
              {title}
            </h1>
            <p className="hidden text-sm text-muted-foreground sm:block">
              {description}
            </p>
          </div>
          <div className="hidden max-w-sm min-w-64 flex-1 items-center gap-2 rounded-lg border border-input bg-background px-3 shadow-xs md:flex">
            <Search className="size-4 text-muted-foreground" />
            <Input
              className="h-8 border-0 px-0 shadow-none focus-visible:ring-0"
              placeholder="Search"
            />
          </div>
          <Button>
            <Plus data-icon="inline-start" />
            New
          </Button>
        </div>
      </header>
      <div className="mx-auto w-full max-w-7xl px-4 py-5 sm:px-6 lg:py-6">
        <Card>
          <CardHeader>
            <CardTitle>{title}</CardTitle>
          </CardHeader>
          <CardContent className="text-sm text-muted-foreground">
            This route is ready for the {title.toLowerCase()} workflow.
          </CardContent>
        </Card>
      </div>
    </>
  )
}

export { PlaceholderRoute }
