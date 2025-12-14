const { invoke } = window.__TAURI__.tauri;

// UI state management
function setLoading(buttonId, loading) {
  const button = document.getElementById(buttonId);
  if (loading) {
    button.classList.add('loading');
    button.setAttribute('disabled', 'true');
  } else {
    button.classList.remove('loading');
    button.removeAttribute('disabled');
  }
}

function showNotification(message, type = 'success') {
  const notification = document.createElement('div');
  notification.className = `notification ${type}`;
  notification.textContent = message;
  document.body.appendChild(notification);
  setTimeout(() => notification.remove(), 3000);
}

async function handleCommand(buttonId, command, args = {}) {
  setLoading(buttonId, true);
  try {
    const result = await invoke(command, args);
    showNotification(result || 'Operation completed successfully');
  } catch (error) {
    showNotification(`Error: ${error}`, 'error');
    console.error(`Command ${command} failed:`, error);
  } finally {
    setLoading(buttonId, false);
  }
}

document.getElementById('open-browser').addEventListener('click', () => {
  handleCommand('open-browser', 'open_browser', { url: 'https://www.rust-lang.org' });
});

document.getElementById('empty-bin').addEventListener('click', () => {
  handleCommand('empty-bin', 'empty_recycle_bin');
});

document.getElementById('organize-files').addEventListener('click', () => {
  handleCommand('organize-files', 'organize_files');
});
