<template>
  <div class="px-4 pt-3">
    <div class="flex w-full">
      <div>
        <fai
          class="text-[150px] dark:text-zinc-300"
          icon="fab fa-apple"
          v-if="props.primary.system_name == 'Darwin'"
        />
        <fai icon="fab fa-linux" v-if="props.primary.system_name == 'Unix'" />
        <fai
          icon="fab fa-windows"
          v-if="props.primary.system_name == 'Windows'"
        />
      </div>
      <div>
        {{ primary }}
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  primary: {
    type: Object,
    required: true,
  },
});
</script>
