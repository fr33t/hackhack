<script setup lang="ts">
import { onMounted, ref } from 'vue';
import axios from '../util/axiostance';
import router from '../router';
import { useTheme } from 'vuetify';
import { isNighttime } from '../util/nighttime';
const theme = useTheme();

const is_targets = ref(0);
const search = ref("");
const email = ref("user@example.com");
const score = ref(0);

const logs = ref([]);
let headers = [
    { title: 'Name', key: 'name' },
    { title: 'Level', key: 'level' },
    { title: 'User Online', key: 'user' },
    { title: 'Pwned', key: 'pwned' }
];

let items = ref([])

const logout = () => {
    localStorage.removeItem("token");
    router.go(0);

};

const download_vpn = () => {
    console.log("Downloading vpn");
};

const change_theme = () => {
    theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
};

const switch_target = (target_name: string) => {
    is_targets.value = 2;
    console.log(target_name);
}

onMounted(() => {
    if (isNighttime()) {
        theme.global.name.value = 'dark';
    }
    axios.post("/auth/test_login").then((res) => {
        if (res.data) {
            email.value = res.data;
        } else {
            router.push("/");
        }
    })
    axios.get("/target/all").then((res) => {
        items.value = res.data;
    })
    axios.get("/target/log").then((res) => {
        logs.value = res.data;
    })

})
</script>

<template>
    <v-card>
        <v-layout>
            <v-navigation-drawer>
                <v-list>
                    <v-list-item :title="email"></v-list-item>
                    <v-list-item :title="score"></v-list-item>
                </v-list>

                <v-divider></v-divider>

                <v-list density="compact" nav>
                    <v-list-item prepend-icon="mdi mdi-bullseye-arrow" title="Targets"
                        @click="is_targets = 0"></v-list-item>
                    <v-list-item prepend-icon="mdi mdi-billboard" title="Billboard"
                        @click="is_targets = 1"></v-list-item>
                    <v-list-item prepend-icon="mdi mdi-theme-light-dark" title="Change Theme"
                        @click="change_theme()"></v-list-item>
                    <v-list-item prepend-icon="mdi mdi-progress-download" title="OpenVPN"
                        @click-once="download_vpn()"></v-list-item>
                    <v-list-item prepend-icon="mdi mdi-logout" title="Logout" @click-once="logout()"></v-list-item>
                </v-list>
            </v-navigation-drawer>

            <v-main v-if="is_targets == 0">
                <v-card flat>
                    <template v-slot:text>
                        <v-text-field v-model="search" label="Search" prepend-inner-icon="mdi-magnify"
                            variant="outlined" hide-details single-line></v-text-field>
                    </template>

                    <v-data-table class="h-90vh" :headers="headers" :items="items" :search="search">
                        <template v-slot:item.name="{ value }">
                            <h2 @click="switch_target(value)" class="text-blue-600 hover:text-black  no-underline ">
                                {{
                                    value
                                }}
                            </h2>
                        </template>
                        <template v-slot:item.level="{ value }">
                            <h3 class="text-yellow-600   no-underline ">
                                {{
                                    value
                                }}
                            </h3>
                        </template>
                        <template v-slot:item.user="{ value }">
                            <h3 class="text-green   no-underline ">{{
                                value
                            }}

                            </h3>
                        </template>
                        <template v-slot:item.pwned="{ value }">
                            <h1 class="text-red  no-underline ">{{
                                value
                                }}

                            </h1>
                        </template>
                    </v-data-table>
                </v-card>
            </v-main>
            <v-main v-else-if="is_targets == 1">
                <v-card flat>
                    <template v-slot:text>
                        <h2>
                            Who pwned this target ?
                        </h2>
                    </template>
                    <v-virtual-scroll class="h-95vh" :items="logs">
                        <template v-slot:default="{ item }">

                            <h3 class="ml-30" v-if="item.flag_type == 'user'">
                                {{ item.created_at }}......... {{ item.email }} has got the {{ item.flag_type }} from {{
                                    item.target_name }}
                                <br>
                            </h3>
                            <h3 class="ml-30  text-red" v-else>
                                {{ item.created_at }}......... {{ item.email }} has got the {{ item.flag_type }} from {{
                                    item.target_name }}
                                <br>
                            </h3>
                        </template>
                    </v-virtual-scroll>
                </v-card>
            </v-main>
            <v-main v-else>
                123
            </v-main>
        </v-layout>
    </v-card>
</template>



<style scoped></style>