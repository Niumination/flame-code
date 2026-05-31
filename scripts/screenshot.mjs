import { chromium } from 'playwright'
import { mkdirSync } from 'fs'
import { resolve } from 'path'

const URL = 'http://localhost:1420/'
const OUT = resolve('screenshots')
mkdirSync(OUT, { recursive: true })

const browser = await chromium.launch({ headless: true })
const page = await browser.newPage({ viewport: { width: 1200, height: 740 } })

page.on('console', msg => {
  if (msg.type() === 'error') console.log('  [PAGE ERROR]', msg.text())
})

await page.goto(URL, { waitUntil: 'networkidle', timeout: 15000 })
await page.waitForTimeout(3000)

const title = await page.title()
console.log('Page title:', title)

const bodyText = await page.evaluate(() => document.body?.innerText?.slice?.(0, 300) || 'no body')
console.log('Body text:', bodyText)

await page.screenshot({ path: `${OUT}/01-full-app.png` })
console.log('✓ 01-full-app.png')

const selectors = [
  ['02-header', 'header[role="banner"]'],
  ['03-sidebar', 'aside[role="navigation"]'],
  ['04-tab-bar', '[role="tablist"]'],
  ['05-explorer', '[class*="explorer" i]'],
  ['06-ai-panel', '[class*="ai-panel" i]'],
  ['07-status-bar', 'footer[role="status"]'],
  ['08-terminal', '[class*="terminal" i]'],
]

for (const [name, sel] of selectors) {
  const el = page.locator(sel).first()
  const visible = await el.isVisible().catch(() => false)
  if (visible) {
    await el.screenshot({ path: `${OUT}/${name}.png` })
    console.log(`✓ ${name}.png (${sel})`)
  } else {
    console.log(`✗ ${name}.png — not found: ${sel}`)
  }
}

await browser.close()
console.log('\nDone — screenshots in', OUT)
