<script setup lang="ts">
import { ref } from 'vue';
const email = ref("");
const verifyCode = ref("");
</script>
<template>
    <div>

        <v-card class="mx-auto my-8" elevation="16" max-width="344">
            <v-form fast-fail @submit.prevent>
                <v-text-field v-model="email" label="First name"></v-text-field>

                <v-text-field v-model="verifyCode" label="Last name"></v-text-field>

                <v-btn class="mt-2" type="submit" block>Submit</v-btn>
            </v-form>
        </v-card>
    </div>
</template>

<style scoped></style>
