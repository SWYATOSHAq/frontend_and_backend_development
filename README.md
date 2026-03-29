# Web Market

Учебный проект — интернет-магазин с разделением ролей.
Бэкенд на **Rust (Actix-web)**, фронтенд на **React (Vite)**.

---

## Стек

| Слой | Технология |
|---|---|
| Бэкенд | Rust, Actix-web |
| Аутентификация | JWT (access + refresh), bcrypt |
| Хранение | In-memory (Vec) |
| Фронтенд | React, Vite, Axios, React Router |

---

## Структура проекта

```
web-market/
├── backend/             # Rust API сервер
│   └── src/
│       ├── handlers/    # Обработчики маршрутов
│       ├── models/      # Структуры данных
│       ├── utils/       # JWT, хеширование
│       ├── routes.rs    # Маршруты
│       ├── state.rs     # Глобальное состояние
│       └── main.rs      # Точка входа
└── frontend-react/      # React приложение
    └── src/
        ├── api/         # axios клиент + функции запросов
        ├── components/  # PrivateRoute
        └── pages/       # Login, Register, Products, Admin
```

---

## Запуск

**Бэкенд** (порт 3000):
```bash
cd backend
cargo run
```

**Фронтенд** (порт 5173):
```bash
cd frontend-react
npm install
npm run dev
```

Открыть: `http://localhost:5173`

---

## API маршруты

### Аутентификация
| Метод | Маршрут | Доступ | Описание |
|---|---|---|---|
| POST | `/api/auth/register` | Все | Регистрация |
| POST | `/api/auth/login` | Все | Вход, возвращает accessToken |
| POST | `/api/auth/refresh` | Все | Обновление токенов |
| GET | `/api/auth/me` | user+ | Текущий пользователь |

### Товары
| Метод | Маршрут | Доступ | Описание |
|---|---|---|---|
| GET | `/api/products` | user+ | Список товаров |
| POST | `/api/products` | seller+ | Создать товар |
| GET | `/api/products/{id}` | user+ | Товар по ID |
| PUT | `/api/products/{id}` | seller+ | Обновить товар |
| DELETE | `/api/products/{id}` | admin | Удалить товар |

### Пользователи
| Метод | Маршрут | Доступ | Описание |
|---|---|---|---|
| GET | `/api/users` | admin | Список пользователей |
| GET | `/api/users/{id}` | admin | Пользователь по ID |
| PATCH | `/api/users/{id}` | admin | Обновить пользователя |
| DELETE | `/api/users/{id}` | admin | Удалить пользователя |

### Загрузка файлов
| Метод | Маршрут | Описание |
|---|---|---|
| POST | `/upload` | Загрузка изображения товара |

---

## Роли (RBAC)

| Роль | Права |
|---|---|
| `user` | Просмотр товаров, `/api/auth/me` |
| `seller` | Все права `user` + создание и редактирование товаров |
| `admin` | Все права `seller` + удаление товаров + управление пользователями |

Роль назначается при регистрации. По умолчанию — `user`.
Роль хранится в JWT-токене и проверяется на каждом защищённом маршруте.

---

## Токены

- **Access token** — живёт 15 минут, хранится в `localStorage`
- **Refresh token** — живёт 7 дней, хранится в `HttpOnly cookie`
- При истечении access token фронтенд автоматически обновляет его через `/api/auth/refresh`

---

## Swagger UI

Документация API доступна по адресу:
`http://localhost:3000/swagger-ui/`
