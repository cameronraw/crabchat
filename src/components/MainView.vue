<script setup lang="ts">
  import Toolbar from "./Toolbar.vue";
  import OrganizationSidebar from "./OrganizationSidebar.vue";
</script>

<template>
  <div class="container mainView">
    <div class="header">
      <div class="optionsSection"><p>Options</p></div>
    </div>
    <div class="mainBody">
      <OrganizationSidebar />
      <Toolbar />
      <div class="messageArea">
        <p>Message Area</p>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
@import "../styles.scss";

.mainView {
  padding: 0;
  height: 100%;
  display: flex;
  flex-direction: column;

  .header {
    display: flex;
    justify-content: flex-end;
    background-color: $lightgreen;
    margin: 0;
    position: fixed;
    top: 0;
    width: 100%;
    height: 50px;

    .optionsSection {
      display: flex;
      justify-content: center;
      flex-direction: column;
      p {
        padding: 15px;
        display: block;
        margin: 0;
      }
    }

  }
  .mainBody {
    display: flex;
    flex-direction: row;
    background-color: $darkgreen;
    color: black;
    flex-grow: 1;
    padding-top: 50px;
  }
}
</style>
