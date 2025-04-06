<template>
  <div class="px-4 pt-3">
    <div class="flex w-full">
      <div class="px-10 items-center place-content-center">
        <fai
          class="text-[200px] xl:text-[300px] dark:text-zinc-300"
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
        <div class="relative overflow-x-auto dark:text-zinc-400">
          <table class="w-full text-md text-left rtl:text-right">
            <tbody>
              <tr>
                <th scope="row" class="px-2 py-2 font-medium whitespace-nowrap">
                  System Name
                </th>
                <td class="px-2 py-4">
                  {{ primary.system_name }}
                </td>
              </tr>
              <tr>
                <th scope="row" class="px-2 py-2 font-medium whitespace-nowrap">
                  OS Version
                </th>
                <td class="px-2 py-4">
                  {{ primary.os_version }}
                </td>
              </tr>
              <tr>
                <th scope="row" class="px-2 py-2 font-medium whitespace-nowrap">
                  Kernal Version
                </th>
                <td class="px-2 py-4">
                  {{ primary.kernel_version }}
                </td>
              </tr>
              <tr>
                <th scope="row" class="px-2 py-2 font-medium whitespace-nowrap">
                  Host Name
                </th>
                <td class="px-2 py-4">
                  {{ primary.host_name }}
                </td>
              </tr>
              <tr>
                <th scope="row" class="px-2 py-2 font-medium whitespace-nowrap">
                  Processes
                </th>
                <td class="px-2 py-4">
                  {{ primary.process_count }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
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
