CREATE TABLE `users` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `name` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `photo` varchar(255) DEFAULT "default.png",
  `verified` BOOLEAN NOT NULL DEFAULT false,
  `password` varchar(255) NOT NULL,
  `role` varchar(36) NOT NULL DEFAULT "ROLE_USER",
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now())
);

CREATE TABLE `products` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `name` varchar(255) NOT NULL,
  `meta_name` varchar(255) NOT NULL,
  `price` int NOT NULL,
  `photo` varchar(255) NOT NULL DEFAULT "default.png",
  `is_available` BOOLEAN NOT NULL DEFAULT true,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now())
);

CREATE TABLE `specifications` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `product_id` varchar(36) NOT NULL,
  `name` varchar(36) NOT NULL,
  `value` varchar(255) NOT NULL
);

CREATE TABLE `order_items` (
  `order_id` varchar(36),
  `product_id` varchar(36) NOT NULL,
  `quantity` int NOT NULL DEFAULT 1,
  `product_name` varchar(255) NOT NULL,
  `product_meta_name` varchar(255) NOT NULL,
  `product_price` int NOT NULL,
  `product_photo` varchar(255) NOT NULL DEFAULT "default.png"
);

CREATE TABLE `orders` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `user_id` varchar(36) NOT NULL,
  `status` ENUM ('COMPLETED', 'SHIPPING_ON_PROCESS', 'PAID', 'WAITING_PAYMENT') NOT NULL DEFAULT ('WAITING_PAYMENT'),
  `awb` varchar(255),
  `shipping_fee` int,
  `total_product_cost` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now())
);

CREATE TABLE `public_data` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `key` varchar(255) NOT NULL,
  `value` varchar(255),
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now())
);

CREATE INDEX `users_email_idx` ON `users` (`email`);

CREATE UNIQUE INDEX `users_index_1` ON `users` (`id`);

CREATE INDEX `product_name_idx` ON `products` (`name`);

CREATE UNIQUE INDEX `products_index_3` ON `products` (`id`);

CREATE UNIQUE INDEX `products_index_4` ON `products` (`meta_name`);

CREATE INDEX `specification_product_id_idx` ON `specifications` (`product_id`);

CREATE UNIQUE INDEX `order_items_index_6` ON `order_items` (`order_id`);

CREATE INDEX `order_status_idx` ON `orders` (`status`);

CREATE INDEX `order_awb_idx` ON `orders` (`awb`);

CREATE INDEX `order_created_at_idx` ON `orders` (`created_at`);

CREATE INDEX `order_updated_at_idx` ON `orders` (`updated_at`);

CREATE UNIQUE INDEX `public_data_index_11` ON `public_data` (`key`);

ALTER TABLE `order_items` ADD FOREIGN KEY (`order_id`) REFERENCES `orders` (`id`);

ALTER TABLE `order_items` ADD FOREIGN KEY (`product_id`) REFERENCES `products` (`id`);

ALTER TABLE `orders` ADD FOREIGN KEY (`user_id`) REFERENCES `users` (`id`);

ALTER TABLE `specifications` ADD FOREIGN KEY (`product_id`) REFERENCES `products` (`id`);
