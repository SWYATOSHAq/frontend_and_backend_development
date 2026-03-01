const API = "http://127.0.0.1:3000/products";

async function loadProducts() {
    const res = await fetch(API);
    const products = await res.json();

    const container = document.getElementById("products-container");
    container.innerHTML = "";

    products.forEach(product => {
        const card = document.createElement("div");
        card.className = "card card--accent";

        card.innerHTML = `
            <h2 class="card__title">${product.name}</h2>
            <div class="card--content">
                <p>Цена: ${product.price}€</p>
                <button onclick="deleteProduct('${product.id}')">
                    Удалить
                </button>
            </div>
        `;

        container.appendChild(card);
    });
}

async function createProduct() {
    const name = document.getElementById("name").value;
    const price = parseFloat(document.getElementById("price").value);

    if (!name || isNaN(price)) {
        alert("Заполни поля");
        return;
    }

    await fetch(API, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({ name, price })
    });

    document.getElementById("name").value = "";
    document.getElementById("price").value = "";

    loadProducts();
}

async function deleteProduct(id) {
    await fetch(`${API}/${id}`, {
        method: "DELETE"
    });

    loadProducts();
}

loadProducts();