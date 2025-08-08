const { invoke } = window.__TAURI__.tauri;

document.getElementById('open-browser').addEventListener('click', () => {
  invoke('open_browser', { url: 'https://www.rust-lang.org' });
});

document.getElementById('empty-bin').addEventListener('click', () => {
  invoke('empty_recycle_bin');
});

document.getElementById('organize-files').addEventListener('click', () => {
  invoke('organize_files');
});
