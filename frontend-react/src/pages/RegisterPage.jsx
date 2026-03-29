import { useState } from "react";
import { useNavigate, Link } from "react-router-dom";
import { register } from "../api/auth";

export default function RegisterPage() {
  const [form, setForm] = useState({ username: "", password: "", age: "", email: "" });
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const handleChange = (e) =>
    setForm({ ...form, [e.target.name]: e.target.value });

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError("");
    try {
      await register({ ...form, age: Number(form.age) });
      navigate("/login");
    } catch (err) {
      setError(err.response?.data?.error || "Ошибка регистрации");
    }
  };

  return (
    <div className="auth-page">
      <div className="auth-card">
        <h2 className="auth-card__title">Регистрация</h2>
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
            <label className="form-label">Email</label>
            <input
              className="form-input"
              name="email"
              type="email"
              placeholder="example@mail.com"
              value={form.email}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label className="form-label">Возраст</label>
            <input
              className="form-input"
              name="age"
              type="number"
              placeholder="Введите возраст"
              value={form.age}
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
          <button className="btn btn--add" type="submit">Зарегистрироваться</button>
        </form>
        <p className="auth-card__footer">
          Уже есть аккаунт? <Link to="/login">Войти</Link>
        </p>
      </div>
    </div>
  );
}
