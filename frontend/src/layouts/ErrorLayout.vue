<template>
  <div class="grid place-items-center items-center justify-center h-screen">
    <div>
      <div>
        <h1 class="text-6xl">Page Not Found</h1>
      </div>
      <div class="text-center mt-7">
        <button
          type="button"
          class="text-dark-theme-primary-text bg-theme-first hover:dark:brightness-110 focus:outline-none font-medium rounded-lg px-6 py-2.5 text-center inline-flex items-center ring-0 cursor-pointer text-md uppercase"
          @click="$router.push('/')"
        >
          <fai icon="fa-arrow-left" class="mr-2" />
          Go Back
        </button>
      </div>
    </div>
  </div>
</template>
