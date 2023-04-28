<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getClient } from "@tauri-apps/api/http";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function auth() {
  const client = await getClient();
  // const response = await client.post();
}
</script>

<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <div class="card">
    <a href="https://slack.com/oauth/v2/authorize?scope=incoming-webhook,commands,chat:write:user&client_id=5186300144243.5171918691895
             " target="_blank">Log into Slack to use CrabChat!</a>
  </div>

  <p>{{ greetMsg }}</p>
</template>
