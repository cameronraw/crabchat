<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import MainView from "./components/MainView.vue";

const bypassLogin = true;
</script>

<template>
  <div v-if="!bypassLogin" class="container">
    <h1>CrabChat</h1>

    <div class="row">
      <p>It's never felt so good to have crabs!</p>
    </div>

    <Greet />
  </div>
  <MainView v-if="bypassLogin"/>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
