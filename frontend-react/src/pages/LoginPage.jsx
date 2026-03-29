import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { login } from "../api/auth";

export default function LoginPage() {
  const [form, setForm] = useState({ username: "", password: "" });
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const handleChange = (e) =>
    setForm({ ...form, [e.target.name]: e.target.value });

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");
    try {
      await login(form);
      navigate("/products");
    } catch (err) {
      setError(err.response?.data?.error || "Ошибка входа");
    }
  };

  return (
    <div className="auth-page">
      <div className="auth-card">
        <h2 className="auth-card__title">Вход</h2>
        <form className="auth-card__form" onSubmit={handleSubmit}>
          <div className="form-group">
            <label className="form-label">Имя пользователя</label>
            <input
              className="form-input"
              name="username"
              placeholder="Введите имя"
              value={form.username}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label className="form-label">Пароль</label>
            <input
              className="form-input"
              name="password"
              type="password"
              placeholder="Введите пароль"
              value={form.password}
              onChange={handleChange}
              required
            />
          </div>
          {error && <p className="error-text">{error}</p>}
          <button className="btn btn--add" type="submit">Войти</button>
        </form>
        <p className="auth-card__footer">
          Нет аккаунта? <Link to="/register">Зарегистрироваться</Link>
        </p>
      </div>
    </div>
  );
}
