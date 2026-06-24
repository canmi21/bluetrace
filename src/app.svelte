<script lang="ts">
  import { isTauri } from '@tauri-apps/api/core'
  import { getCurrentWindow } from '@tauri-apps/api/window'

  const appWindow = isTauri() ? getCurrentWindow() : null

  function dragWindow(node: HTMLElement) {
    function startWindowDrag(event: MouseEvent) {
      if (!appWindow || event.button !== 0) {
        return
      }

      void appWindow.startDragging()
    }

    node.addEventListener('mousedown', startWindowDrag)

    return {
      destroy() {
        node.removeEventListener('mousedown', startWindowDrag)
      },
    }
  }
</script>

<svelte:head>
  <title>Bluetrace</title>
  <meta name="theme-color" content="#141414" />
</svelte:head>

<main class="min-h-screen overflow-hidden bg-background text-foreground">
  <section class="flex min-h-screen flex-col">
    <header
      class="h-8 shrink-0 border-b border-line bg-chrome"
      use:dragWindow
    >
      <div class="h-px bg-edge-highlight"></div>
      <div class="flex h-[31px] items-center justify-center px-20">
        <p class="select-none truncate text-center text-[13px] font-medium leading-none text-muted">
          Bluetrace
        </p>
      </div>
    </header>

    <section class="min-h-0 flex-1 bg-background"></section>
  </section>
</main>
