import apiClient from "./index";

export const register = (data) =>
  apiClient.post("/auth/register", data);

export const login = async (data) => {
  const response = await apiClient.post("/auth/login", data);
  const token = response.data.accessToken;
  localStorage.setItem("accessToken", token);
  // декодируем payload токена и сохраняем роль
  const payload = JSON.parse(atob(token.split(".")[1]));
  localStorage.setItem("role", payload.role);
  return response;
};

export const refresh = () =>
  apiClient.post("/auth/refresh");

export const getMe = () =>
  apiClient.get("/auth/me");

export const logout = () => {
  localStorage.removeItem("accessToken");
  localStorage.removeItem("role");
};

export const getRole = () => localStorage.getItem("role") || "";
