import client from "axios";

const axios = client.create({
  baseURL: "http://127.0.0.1:8000",
  timeout: 2000,
  headers: {
    "Content-Type": "application/json",
  },
});

// 请求拦截器
axios.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("token"); // 从 localStorage 中获取 Token
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// 响应拦截器
axios.interceptors.response.use(
  (response) => response,
  (error) => {
    // 处理错误（例如，Token 过期或其他错误）
    if (error.response.status === 401) {
      // 清除无效的 Token
      localStorage.removeItem("token");
    }
    return Promise.reject(error);
  }
);

export default axios;
