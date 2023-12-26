const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;
let usage;

async function calculate_usage_memory() {
  usage.textContent = await invoke('calculate_usage_memory') + "%";
}

async function clean() {
  await invoke('clean');
}

window.addEventListener("DOMContentLoaded", () => {
  usage = document.querySelector(".usage");
  document.querySelector("#from").addEventListener("submit", (e) => {
      e.preventDefault();
      clean();
  });

  document.getElementById("close")
          .addEventListener("click", () => appWindow.close())
  document.getElementById("collapse")
          .addEventListener("click", () => appWindow.minimize())

  setTimeout(function run() {
    calculate_usage_memory();
    setTimeout(run, 1000);
  }, 1000);
});

