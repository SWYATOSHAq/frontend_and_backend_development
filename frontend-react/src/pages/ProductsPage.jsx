import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getProducts, createProduct, updateProduct, deleteProduct, uploadImage } from "../api/products";
import { logout } from "../api/auth";

export default function ProductsPage() {
  const [products, setProducts] = useState([]);
  const [form, setForm] = useState({ name: "", description: "", price: "", quantity: "", category: "", image_url: null });
  const [editingProduct, setEditingProduct] = useState(null);
  const [error, setError] = useState("");
  const [imagePreview, setImagePreview] = useState(null);
  const navigate = useNavigate();

  const fetchProducts = async () => {
    try {
      const res = await getProducts();
      setProducts(res.data);
    } catch {
      setError("Ошибка загрузки товаров");
    }
  };

  useEffect(() => {
    fetchProducts();
  }, []);

  const handleChange = (e) =>
    setForm({ ...form, [e.target.name]: e.target.value });

  const handleImageChange = async (e) => {
    const file = e.target.files[0];
    if (!file) return;
    try {
      const res = await uploadImage(file);
      const url = res.data.url;
      setForm((prev) => ({ ...prev, image_url: url }));
      setImagePreview("http://localhost:3000" + url);
    } catch {
      setError("Ошибка загрузки изображения");
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");
    const payload = { ...form, price: Number(form.price), quantity: Number(form.quantity) };
    try {
      await createProduct(payload);
      setForm({ name: "", description: "", price: "", quantity: "", category: "", image_url: null });
      setImagePreview(null);
      fetchProducts();
    } catch (err) {
      setError(err.response?.data?.error || "Ошибка сохранения");
    }
  };

  const handleEditSubmit = async (e) => {
    e.preventDefault();
    setError("");
    const payload = { ...form, price: Number(form.price), quantity: Number(form.quantity) };
    try {
      await updateProduct(editingProduct.id, payload);
      setEditingProduct(null);
      setForm({ name: "", description: "", price: "", quantity: "", category: "", image_url: null });
      setImagePreview(null);
      fetchProducts();
    } catch (err) {
      setError(err.response?.data?.error || "Ошибка сохранения");
    }
  };

  const handleEdit = (product) => {
    setEditingProduct(product);
    setForm({ name: product.name, description: product.description, price: product.price, quantity: product.quantity, category: product.category, image_url: product.image_url || null });
    setImagePreview(product.image_url ? "http://localhost:3000" + product.image_url : null);
  };

  const handleDelete = async (id) => {
    try {
      await deleteProduct(id);
      fetchProducts();
    } catch {
      setError("Ошибка удаления");
    }
  };

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <div className="layout">
      {/* Sidebar — форма добавления */}
      <aside className="sidebar">
        <div className="sidebar__header">
          <span className="sidebar__title">Добавить товар</span>
        </div>
        <form className="sidebar__form" onSubmit={handleSubmit}>
          <div className="form-group">
            <label className="form-label">Название</label>
            <input
              className="form-input"
              name="name"
              placeholder="Название товара"
              value={form.name}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label className="form-label">Описание</label>
            <textarea
              className="form-input form-textarea"
              name="description"
              placeholder="Описание товара"
              value={form.description}
              onChange={handleChange}
            />
          </div>
          <div className="form-group">
            <label className="form-label">Категория</label>
            <input
              className="form-input"
              name="category"
              placeholder="Например: электроника"
              value={form.category}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label className="form-label">Цена (₽)</label>
            <input
              className="form-input"
              name="price"
              type="number"
              placeholder="0"
              value={form.price}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label className="form-label">Количество</label>
            <input
              className="form-input"
              name="quantity"
              type="number"
              placeholder="0"
              value={form.quantity}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label className="form-label">Фото товара</label>
            <input
              className="form-input"
              type="file"
              accept="image/*"
              onChange={handleImageChange}
            />
            {imagePreview && (
              <img src={imagePreview} alt="превью" style={{ marginTop: 8, borderRadius: 8, width: "100%", height: 120, objectFit: "cover" }} />
            )}
          </div>
          {error && <p className="error-text">{error}</p>}
          <button className="btn btn--add" type="submit">Добавить товар</button>
        </form>
      </aside>

      {/* Main — список товаров */}
      <main className="main">
        <div className="main__header">
          <h1 className="main__title">Товары</h1>
          <span className="main__count">{products.length}</span>
          <button className="btn btn--logout" style={{ marginLeft: "auto" }} onClick={handleLogout}>
            Выйти
          </button>
        </div>

        <div className="products-grid">
          {products.length === 0 ? (
            <div className="empty-state">
              <span className="empty-state__icon">📦</span>
              <p className="empty-state__text">Товаров пока нет</p>
            </div>
          ) : (
            products.map((p) => (
              <div className="card" key={p.id}>
                <div className="card__image-wrapper">
                  {p.image_url
                    ? <img className="card__image" src={"http://localhost:3000" + p.image_url} alt={p.name} />
                    : <span className="card__image-placeholder">🛍️</span>
                  }
                </div>
                <div className="card__body">
                  <h3 className="card__title">{p.name}</h3>
                  <div className="card__meta">
                    <span className="card__category">{p.category}</span>
                    <span className="card__stock">В наличии: {p.quantity} шт.</span>
                  </div>
                  <p className="card__description">{p.description}</p>
                  <p className="card__price">{p.price} ₽</p>
                </div>
                <div className="card__actions">
                  <button className="btn btn--edit" onClick={() => handleEdit(p)}>
                    Редактировать
                  </button>
                  <button className="btn btn--delete" onClick={() => handleDelete(p.id)}>
                    Удалить
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </main>

      {/* Modal — редактирование */}
      <div className={`modal ${editingProduct ? "modal--open" : ""}`}>
        <div className="modal__overlay" onClick={() => setEditingProduct(null)} />
        <div className="modal__content">
          <button className="modal__close" onClick={() => setEditingProduct(null)}>✕</button>
          <h2 className="modal__title">Редактировать товар</h2>
          <form className="modal__form" onSubmit={handleEditSubmit}>
            <div className="form-group">
              <label className="form-label">Название</label>
              <input
                className="form-input"
                name="name"
                placeholder="Название товара"
                value={form.name}
                onChange={handleChange}
                required
              />
            </div>
            <div className="form-group">
              <label className="form-label">Описание</label>
              <textarea
                className="form-input form-textarea"
                name="description"
                placeholder="Описание товара"
                value={form.description}
                onChange={handleChange}
              />
            </div>
            <div className="form-group">
              <label className="form-label">Категория</label>
              <input
                className="form-input"
                name="category"
                placeholder="Например: электроника"
                value={form.category}
                onChange={handleChange}
                required
              />
            </div>
            <div className="form-group">
              <label className="form-label">Цена (₽)</label>
              <input
                className="form-input"
                name="price"
                type="number"
                placeholder="0"
                value={form.price}
                onChange={handleChange}
                required
              />
            </div>
            <div className="form-group">
              <label className="form-label">Количество</label>
              <input
                className="form-input"
                name="quantity"
                type="number"
                placeholder="0"
                value={form.quantity}
                onChange={handleChange}
                required
              />
            </div>
            <div className="form-group">
              <label className="form-label">Фото товара</label>
              <input
                className="form-input"
                type="file"
                accept="image/*"
                onChange={handleImageChange}
              />
              {imagePreview && (
                <img src={imagePreview} alt="превью" style={{ marginTop: 8, borderRadius: 8, width: "100%", height: 120, objectFit: "cover" }} />
              )}
            </div>
            {error && <p className="error-text">{error}</p>}
            <div className="modal__actions">
              <button className="btn btn--save" type="submit">Сохранить</button>
              <button className="btn btn--cancel" type="button" onClick={() => setEditingProduct(null)}>
                Отмена
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
