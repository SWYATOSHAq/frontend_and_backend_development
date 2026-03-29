import axios from "axios";
import apiClient from "./index";

export const uploadImage = (file) => {
  const formData = new FormData();
  formData.append("file", file);
  return axios.post("http://localhost:3000/upload", formData, {
    withCredentials: true,
  });
};

export const getProducts = () =>
  apiClient.get("/products");

export const createProduct = (data) =>
  apiClient.post("/products", data);

export const getProduct = (id) =>
  apiClient.get(`/products/${id}`);

export const updateProduct = (id, data) =>
  apiClient.put(`/products/${id}`, data);

export const deleteProduct = (id) =>
  apiClient.delete(`/products/${id}`);
