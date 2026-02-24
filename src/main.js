const { invoke } = window.__TAURI__?.tauri ?? { invoke: null };

function setLoading(buttonId, loading) {
  const button = document.getElementById(buttonId);
  if (loading) {
    button.classList.add('loading');
    button.disabled = true;
  } else {
    button.classList.remove('loading');
    button.disabled = false;
  }
}

function showNotification(message, type = 'success') {
  const container = document.getElementById('notifications');
  const notification = document.createElement('div');
  notification.className = `notification ${type}`;
  notification.setAttribute('role', 'alert');
  notification.textContent = message;

  const closeBtn = document.createElement('button');
  closeBtn.className = 'notification-close';
  closeBtn.setAttribute('aria-label', 'Dismiss notification');
  closeBtn.textContent = '×';
  closeBtn.addEventListener('click', () => notification.remove());
  notification.appendChild(closeBtn);

  container.appendChild(notification);
  const timer = setTimeout(() => notification.remove(), 5000);
  closeBtn.addEventListener('click', () => clearTimeout(timer));
}

async function handleCommand(buttonId, command, args = {}) {
  if (!invoke) {
    showNotification('Tauri runtime not available. Please run this app via Tauri.', 'error');
    return;
  }
  setLoading(buttonId, true);
  try {
    const result = await invoke(command, args);
    const message = (result !== null && result !== undefined && result !== '')
      ? result
      : 'Operation completed successfully';
    showNotification(message);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    showNotification(`Error: ${errorMessage}`, 'error');
    console.error(`Command ${command} failed:`, error);
  } finally {
    setLoading(buttonId, false);
  }
}

async function confirmAndRun(buttonId, command, args, confirmMessage) {
  if (!window.confirm(confirmMessage)) {
    return;
  }
  await handleCommand(buttonId, command, args);
}

document.getElementById('open-browser').addEventListener('click', () => {
  handleCommand('open-browser', 'open_browser', { url: 'https://www.rust-lang.org' });
});

document.getElementById('empty-bin').addEventListener('click', () => {
  confirmAndRun(
    'empty-bin',
    'empty_recycle_bin',
    {},
    'Are you sure you want to permanently empty the recycle bin? This cannot be undone.'
  );
});

document.getElementById('organize-files').addEventListener('click', () => {
  handleCommand('organize-files', 'organize_files');
});
