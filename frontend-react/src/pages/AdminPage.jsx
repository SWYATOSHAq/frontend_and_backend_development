import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { getUsers, deleteUser } from "../api/users";

export default function AdminPage() {
  const [users, setUsers] = useState([]);
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const fetchUsers = async () => {
    try {
      const res = await getUsers();
      setUsers(res.data);
    } catch {
      setError("Ошибка загрузки пользователей");
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleDelete = async (id) => {
    try {
      await deleteUser(id);
      fetchUsers();
    } catch {
      setError("Ошибка удаления");
    }
  };

  const roleLabel = (role) => {
    if (role === "admin") return "Администратор";
    if (role === "seller") return "Продавец";
    return "Пользователь";
  };

  return (
    <div className="layout">
      <aside className="sidebar">
        <div className="sidebar__header">
          <span className="sidebar__title">Панель админа</span>
        </div>
        <div className="sidebar__form">
          <button className="btn btn--add" onClick={() => navigate("/products")}>
            ← К товарам
          </button>
        </div>
      </aside>

      <main className="main">
        <div className="main__header">
          <h1 className="main__title">Пользователи</h1>
          <span className="main__count">{users.length}</span>
        </div>

        {error && <p className="error-text">{error}</p>}

        <div className="products-grid">
          {users.length === 0 ? (
            <div className="empty-state">
              <span className="empty-state__icon">👤</span>
              <p className="empty-state__text">Пользователей нет</p>
            </div>
          ) : (
            users.map((u) => (
              <div className="card" key={u.id}>
                <div className="card__image-wrapper">
                  <span className="card__image-placeholder">👤</span>
                </div>
                <div className="card__body">
                  <h3 className="card__title">{u.username}</h3>
                  <div className="card__meta">
                    <span className="card__category">{roleLabel(u.role)}</span>
                    <span className="card__stock">Возраст: {u.age}</span>
                  </div>
                  <p className="card__description">{u.email}</p>
                </div>
                <div className="card__actions">
                  <button className="btn btn--delete" onClick={() => handleDelete(u.id)}>
                    Удалить
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </main>
    </div>
  );
}
