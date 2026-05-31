# Sample Search Procedure

Deskripsi
Prosedur contoh untuk melakukan pencarian Google dan menyimpan hasilnya.

Tags
- search
- sample
- demo

Trigger
- manual

## Langkah

```js
// Buka browser dan navigasi ke Google
const url = 'https://www.google.com';
browser.goto(url);
```

```js
// Ketik query pencarian
const query = 'Tauri desktop app tutorial';
browser.type('input[name="q"]', query);
browser.click('input[name="btnK"]');
```

```js
// Tunggu hasil pencarian loaded
browser.wait(2000);
```

```js
// Ambil screenshot hasil pencarian
browser.screenshot('search-results');
```

```js
// Ekstrak links dari hasil pencarian
const links = browser.extract('links');
```

```js
// Simpan hasil ke file
files.json('search-results', { query, links });
```
