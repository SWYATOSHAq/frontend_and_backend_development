import apiClient from "./index";

export const register = (data) =>
  apiClient.post("/auth/register", data);

export const login = async (data) => {
  const response = await apiClient.post("/auth/login", data);
  localStorage.setItem("accessToken", response.data.accessToken);
  return response;
};

export const refresh = () =>
  apiClient.post("/auth/refresh");

export const getMe = () =>
  apiClient.get("/auth/me");

export const logout = () => {
  localStorage.removeItem("accessToken");
};
