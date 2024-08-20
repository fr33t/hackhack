<script setup lang="ts">
import { onMounted, ref } from 'vue';
import axios from '../util/axiostance';
import router from '../router';
import { useTheme } from 'vuetify';
import { isNighttime } from '../util/nighttime';
const theme = useTheme();
const this_target = ref({});
const is_targets = ref(0);
const search = ref("");
const email = ref("user@example.com");
const score = ref(0);
const snackbar = ref(false);
const message = ref("");
const logs = ref([]);
const user_flag = ref("");
const root_flag = ref("");

const submit_flag = (flag_type: string) => {
    let flag;
    if (flag_type == "user") {
        flag = user_flag.value;
    } else {
        flag = root_flag.value;
    }
    axios.post("/target/submit", { flag: flag, flag_type: flag_type, target_name: this_target.value.name }).then((res) => {
        message.value = res.data;
        snackbar.value = true;

    });
}
let headers = [
    { title: 'Name', key: 'name' },
    { title: "Type", key: "typ" },
    { title: 'Level', key: 'level' },
    // { title: 'User Online', key: 'user' },
    { title: 'Pwned', key: 'pwned' }
];

let items = ref([])

const logout = () => {
    localStorage.removeItem("token");
    router.go(0);


};

const download_vpn = () => {
    console.log("Downloading vpn");
    axios.get("/auth/vpn").then((res) => {
        // 创建一个 Blob URL
        const blob = new Blob([res.data], { type: res.headers['content-type'] });
        const downloadUrl = window.URL.createObjectURL(blob);

        // 创建一个隐藏的下载链接
        const link = document.createElement('a');
        link.href = downloadUrl;
        link.download = email.value.replace("@", "_").replace(".", "_") + ".ovpn"; // 设置下载文件名

        // 模拟点击链接以启动下载
        document.body.appendChild(link);
        link.click();

        // 清理操作
        document.body.removeChild(link);
        window.URL.revokeObjectURL(downloadUrl);
    })
};

const change_theme = () => {
    theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
};


const switch_target = (target_name: string) => {
    is_targets.value = 2;
    axios.get(`/target/get/${target_name}`).then((res) => {
        this_target.value = res.data;
    })
}

const copy_ip = (ip: string) => {
    navigator.clipboard.writeText(ip);
    message.value = `${ip} copied`;
    snackbar.value = true;
}

onMounted(() => {
    if (isNighttime()) {
        theme.global.name.value = 'dark';
    }
    axios.post("/auth/test_login").then((res) => {
        if (res.data) {
            email.value = res.data.email;
            score.value = res.data.score;
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
                        <template v-slot:item.typ="{ value }">
                            <h2 class="text-black no-underline ">
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
                        <!-- <template v-slot:item.user="{ value }">
                            <h3 class="text-green   no-underline ">{{
                                value
                            }}

                            </h3>
                        </template> -->
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
                <v-card class="ml-4 h-100vh">
                    <v-card-text>
                        <div>
                            <p class="text-h4 font-weight-black text-blue-600 hover:text-black  no-underline">{{
                                this_target.name }}
                            </p>
                            <h3 class="mt-5 text-black no-underline">Type: {{ this_target.typ }}</h3>
                            <!-- <h2 class="mt-2 text-green   no-underline">Online: {{ this_target.user }} </h2> -->
                            <div class="flex mt-2">
                                <h3 class="text-yellow-600   no-underline ">Level: {{ this_target.level }}</h3>
                                <h3 class="ml-5 text-red  no-underline">Pwned: {{ this_target.pwned }}</h3>
                            </div>
                        </div>



                        <div class="ml-10 mt-5 text-medium-emphasis">
                            {{ this_target.description }}
                        </div>
                    </v-card-text>

                    <v-card-actions>
                        <v-btn @click="copy_ip(this_target.ip)" color="deep-purple-accent-4" :text="this_target.ip"
                            variant="text"></v-btn>
                    </v-card-actions>
                    <div class="flex m-2">


                        <v-text-field label="User Flag" v-model="user_flag" max-width="400"></v-text-field><v-btn
                            class="ml-5" rounded="xl" size="large" @click="submit_flag('user')">Submit
                        </v-btn>

                    </div>
                    <div class="flex m-2">

                        <v-text-field label="Root Flag" v-model="root_flag" max-width="400"></v-text-field><v-btn
                            class="ml-5" rounded="xl" size="large" @click="submit_flag('root')">Submit
                        </v-btn>
                    </div>

                </v-card>
            </v-main>
        </v-layout>
    </v-card>
    <div class="text-center ma-2">
        <v-snackbar v-model="snackbar">
            {{ message }}

            <template v-slot:actions>
                <v-btn color="pink" variant="text" @click="snackbar = false">
                    Close
                </v-btn>
            </template>
        </v-snackbar>
    </div>
</template>



<style scoped></style>