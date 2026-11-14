import http from "k6/http";
import { check, sleep } from "k6";

export const options = {
  stages: [
    { duration: "20s", target: 30 },
    { duration: "40s", target: 100 },
    { duration: "20s", target: 0 },
  ],
  thresholds: {
    http_req_duration: ["p(95)<120"],
    http_req_failed: ["rate<0.01"],
  },
};

const BASE_URL = "http://localhost:8080/api";

export default function () {
  const resCatalogo = http.get(`${BASE_URL}/catalogo`);
  check(resCatalogo, { "catálogo OK": (r) => r.status === 200 });

  const payload = JSON.stringify({
    comprador_institucional: "Hotel San Salvador Institutional",
    items: [
      {
        producto_id: "prod_test_01",
        pyme_id: "pyme_test_01",
        cantidad: 5,
        subtotal: 227.5,
      },
    ],
  });

  const params = { headers: { "Content-Type": "application/json" } };
  const resOrden = http.post(`${BASE_URL}/ordenes`, payload, params);
  check(resOrden, { "orden consolidada creada": (r) => r.status === 200 });

  sleep(0.5);
}
