<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let logicGateContent = "INITIALIZING WHITE ROOM CORE...";
  let status = "OPTIMIZING...";
  let heartRate = 0;
  let latency = 0;

  async function fetchLogicGate() {
    try {
      logicGateContent = await invoke("get_logic_gate", { id: "START_001" });
    } catch (e) {
      console.error(e);
    }
  }

  onMount(() => {
    fetchLogicGate();
    // Start biometric feedback loop mock
    const interval = setInterval(() => {
        heartRate = Math.floor(Math.random() * 20) + 70;
        latency = Math.floor(Math.random() * 100);
    }, 1000);
    return () => clearInterval(interval);
  });
</script>

<main class="white-room">
  <div class="sidebar left">
    <div class="metric">BPM: {heartRate}</div>
    <div class="metric">LATENCY: {latency}ms</div>
    <div class="metric">CYCLE: 01/36</div>
  </div>

  <div class="center-stage">
    <div class="header">W.R. CORE // OVERSEER ACTIVE</div>

    <div class="logic-gate">
      <div class="content">
        {logicGateContent}
      </div>
      <div class="input-zone">
        <input type="text" placeholder="DERIVE SOLUTION..." autofocus />
      </div>
    </div>
  </div>

  <div class="sidebar right">
    <div class="status-box">
      <div class="label">SYSTEM STATUS</div>
      <div class="value">{status}</div>
    </div>
    <div class="warning-zone">
        NO EXIT DETECTED
    </div>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background-color: white;
    color: black;
    font-family: 'Courier New', Courier, monospace;
    overflow: hidden;
    cursor: none; /* Hide cursor for immersion */
  }

  .white-room {
    display: flex;
    height: 100vh;
    width: 100vw;
    border: 20px solid black;
    box-sizing: border-box;
  }

  .sidebar {
    width: 200px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    border-right: 2px solid black;
  }

  .sidebar.right {
    border-right: none;
    border-left: 2px solid black;
  }

  .metric {
    font-weight: bold;
    font-size: 1.2rem;
    margin-bottom: 10px;
  }

  .center-stage {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    padding: 40px;
  }

  .header {
    font-size: 1.5rem;
    font-weight: 900;
    border-bottom: 4px solid black;
    padding-bottom: 10px;
    margin-bottom: 40px;
    text-align: center;
  }

  .logic-gate {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .content {
    font-size: 1.2rem;
    max-width: 600px;
    margin-bottom: 40px;
    line-height: 1.6;
  }

  .input-zone {
    width: 100%;
    max-width: 500px;
  }

  input {
    width: 100%;
    border: 2px solid black;
    padding: 15px;
    font-family: inherit;
    font-size: 1.2rem;
    outline: none;
  }

  .status-box {
    border: 2px solid black;
    padding: 10px;
    text-align: center;
  }

  .label {
    font-size: 0.8rem;
    margin-bottom: 5px;
  }

  .value {
    font-weight: bold;
  }

  .warning-zone {
    background-color: black;
    color: white;
    padding: 10px;
    text-align: center;
    font-weight: bold;
  }
</style>
