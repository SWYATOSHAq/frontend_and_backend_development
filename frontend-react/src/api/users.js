import apiClient from "./index";

export const getUsers = () => apiClient.get("/users");
export const getUserById = (id) => apiClient.get(`/users/${id}`);
export const updateUser = (id, data) => apiClient.patch(`/users/${id}`, data);
export const deleteUser = (id) => apiClient.delete(`/users/${id}`);
