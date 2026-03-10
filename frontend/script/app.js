const API = "http://127.0.0.1:3000/products";
const UPLOAD = "http://127.0.0.1:3000/upload";

function escapeHtml(str) {
    const div = document.createElement("div");
    div.textContent = str ?? "";
    return div.innerHTML;
}

// Превью выбранного изображения в форме создания
function previewImage(input) {
    const preview = document.getElementById("image-preview");
    const label = document.getElementById("file-upload-text");

    if (input.files && input.files[0]) {
        const reader = new FileReader();
        reader.onload = e => {
            preview.src = e.target.result;
            preview.style.display = "block";
        };
        reader.readAsDataURL(input.files[0]);
        label.textContent = input.files[0].name;
    } else {
        preview.style.display = "none";
        label.textContent = "Выбрать файл PNG";
    }
}

async function loadProducts() {
    const res = await fetch(API);
    const products = await res.json();

    const container = document.getElementById("products-container");
    const countEl = document.getElementById("products-count");

    container.innerHTML = "";
    countEl.textContent = products.length > 0 ? `${products.length} товаров` : "";

    if (products.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <span class="empty-state__icon">🛒</span>
                <p class="empty-state__text">Товары пока не добавлены</p>
            </div>
        `;
        return;
    }

    products.forEach(product => {
        const card = document.createElement("div");
        card.className = "card";

        const imageHtml = product.image_url
            ? `<img class="card__image" src="http://127.0.0.1:3000${product.image_url}" alt="${escapeHtml(product.name)}">`
            : `<span class="card__image-placeholder">📦</span>`;

        card.innerHTML = `
            <div class="card__image-wrapper">
                ${imageHtml}
            </div>
            <div class="card__body">
                <h3 class="card__title">${escapeHtml(product.name)}</h3>
                <div class="card__meta">
                    <span class="card__category">${escapeHtml(product.category)}</span>
                    <span class="card__stock">На складе: ${product.quantity} шт.</span>
                </div>
                <p class="card__price">€${product.price.toFixed(2)}</p>
                <p class="card__description">${escapeHtml(product.description) || "Описание не указано"}</p>
            </div>
            <div class="card__actions">
                <button class="btn btn--edit" onclick="openEditModal('${product.id}')">Редактировать</button>
                <button class="btn btn--delete" onclick="deleteProduct('${product.id}')">Удалить</button>
            </div>
        `;

        container.appendChild(card);
    });
}

async function createProduct(event) {
    event.preventDefault();

    const name = document.getElementById("name").value.trim();
    const price = parseFloat(document.getElementById("price").value);
    const quantity = parseInt(document.getElementById("quantity").value);
    const category = document.getElementById("category").value.trim();
    const description = document.getElementById("description").value.trim();
    const fileInput = document.getElementById("image");

    // Загружаем картинку если выбрана
    let image_url = null;
    if (fileInput.files[0]) {
        const formData = new FormData();
        formData.append("file", fileInput.files[0]);

        const uploadRes = await fetch(UPLOAD, {
            method: "POST",
            body: formData
        });
        const uploadData = await uploadRes.json();
        image_url = uploadData.url;
    }

    await fetch(API, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, price, quantity, category, description, image_url })
    });

    event.target.reset();
    document.getElementById("image-preview").style.display = "none";
    document.getElementById("file-upload-text").textContent = "Выбрать файл PNG";

    loadProducts();
}

async function deleteProduct(id) {
    if (!confirm("Удалить этот товар?")) return;

    await fetch(`${API}/${id}`, { method: "DELETE" });
    loadProducts();
}

async function openEditModal(id) {
    const res = await fetch(`${API}/${id}`);
    const product = await res.json();

    document.getElementById("edit-id").value = product.id;
    document.getElementById("edit-name").value = product.name;
    document.getElementById("edit-price").value = product.price;
    document.getElementById("edit-quantity").value = product.quantity;
    document.getElementById("edit-category").value = product.category || "";
    document.getElementById("edit-description").value = product.description || "";
    document.getElementById("edit-current-image").value = product.image_url || "";
    document.getElementById("edit-image").value = "";

    const preview = document.getElementById("edit-image-preview");
    if (product.image_url) {
        preview.src = `http://127.0.0.1:3000${product.image_url}`;
        preview.style.display = "block";
    } else {
        preview.style.display = "none";
    }

    document.getElementById("edit-modal").classList.add("modal--open");
}

function previewEditImage(input) {
    const preview = document.getElementById("edit-image-preview");
    if (input.files && input.files[0]) {
        const reader = new FileReader();
        reader.onload = e => {
            preview.src = e.target.result;
            preview.style.display = "block";
        };
        reader.readAsDataURL(input.files[0]);
    }
}

function closeModal() {
    document.getElementById("edit-modal").classList.remove("modal--open");
}

async function saveProduct() {
    const id = document.getElementById("edit-id").value;
    const name = document.getElementById("edit-name").value.trim();
    const price = parseFloat(document.getElementById("edit-price").value);
    const quantity = parseInt(document.getElementById("edit-quantity").value);
    const category = document.getElementById("edit-category").value.trim();
    const description = document.getElementById("edit-description").value.trim();
    const fileInput = document.getElementById("edit-image");

    if (!name || isNaN(price) || isNaN(quantity)) {
        alert("Заполните обязательные поля: Название, Цена, Количество");
        return;
    }

    let image_url = document.getElementById("edit-current-image").value || null;

    if (fileInput.files[0]) {
        const formData = new FormData();
        formData.append("file", fileInput.files[0]);
        const uploadRes = await fetch(UPLOAD, { method: "POST", body: formData });
        const uploadData = await uploadRes.json();
        image_url = uploadData.url;
    }

    await fetch(`${API}/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, price, quantity, category, description, image_url })
    });

    closeModal();
    loadProducts();
}

document.addEventListener("keydown", e => {
    if (e.key === "Escape") closeModal();
});

loadProducts();
