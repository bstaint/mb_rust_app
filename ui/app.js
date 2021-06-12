const vm = new Vue({
  el: '#app',
  data: () => ({
    selected: "main-page",
    version: "0.0.1",
    popupVisible: false,
    system_stat: {
      cpu_load: 0,
      memery_free: 0,
    }
  }),
  methods: {
    handleClick: () => {
      vm.selected = "about-page";
      vm.$indicator.open();
      update_website();
    },
    handleSystemStat: () => {
      vm.selected = "main-page";
      system_stat();
    }
  }
});

