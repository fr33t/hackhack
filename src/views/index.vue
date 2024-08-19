<script setup lang="ts">
import { ref } from 'vue';
const email = ref("");
const verifyCode = ref("");
const error_email = ref(false);
const error_text = ref("");
const loading = ref(false);

const send_code = () => {
    var reg = /^([a-zA-Z]|[0-9])(\w|\-)+@[a-zA-Z0-9]+\.([a-zA-Z]{2,4})$/;
    if (reg.test(email.value)) {

    } else {
        error_text.value = "Invalid email address!";
        error_email.value = true;
    }
};
const check_code = () => {
    console.log(verifyCode.value);
    loading.value = true;
    setTimeout(() => {
        loading.value = false
    }, 2000)
}
</script>
<template>
    <div>
        <v-card class="mx-auto my-10" hover max-width=" 344">
            <center class="mt-4">
                <v-btn @click="send_code()" variant="plain">
                    hackhack
                </v-btn>
            </center>

            <v-form fast-fail @submit.prevent class="m-4">

                <v-text-field v-model="email" type="email" placeholder="user@example.com" label="Email" class="mt-2"
                    variant="outlined"></v-text-field>
                <v-otp-input v-model="verifyCode" length="5" @finish="check_code()" :loading="loading"
                    variant="underlined"></v-otp-input>

            </v-form>
        </v-card>
        <v-snackbar v-model="error_email" :timeout="2000">
            {{ error_text }}
        </v-snackbar>
    </div>
</template>

<style scoped></style>
