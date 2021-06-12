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
    handleClick: function() {
      vm.$indicator.open();
      update_website();
    },
    handleSystemStat: function() {
      system_stat();
    }
  }
});

