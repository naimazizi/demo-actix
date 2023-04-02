CREATE TABLE IF NOT EXISTS `users` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `name` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `photo` varchar(255) DEFAULT "default.png",
  `verified` BOOLEAN NOT NULL DEFAULT false,
  `password` varchar(255) NOT NULL,
  `role` varchar(36) NOT NULL DEFAULT "ROLE_USER",
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  INDEX `users_email_idx` (`email`)
);

CREATE TABLE IF NOT EXISTS `products` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `name` varchar(255) NOT NULL,
  `meta_name` varchar(255) NOT NULL,
  `price` int NOT NULL,
  `photo` varchar(255) NOT NULL DEFAULT "default.png",
  `is_available` BOOLEAN NOT NULL DEFAULT true,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  INDEX `products_name_idx` (`name`),
  UNIQUE INDEX `products_meta_name_idx` (`meta_name`)
);

CREATE TABLE IF NOT EXISTS `specifications` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `product_id` varchar(36) NOT NULL,
  `name` varchar(36) NOT NULL,
  `value` varchar(255) NOT NULL,
  INDEX `specification_product_id_idx` (`product_id`)
);

CREATE TABLE IF NOT EXISTS `order_items` (
  `order_id` varchar(36) DEFAULT (UUID()),
  `product_id` varchar(36) NOT NULL,
  `quantity` int NOT NULL DEFAULT 1,
  `product_name` varchar(255) NOT NULL,
  `product_meta_name` varchar(255) NOT NULL,
  `product_price` int NOT NULL,
  `product_photo` varchar(255) NOT NULL DEFAULT "default.png",
  UNIQUE INDEX `order_items_order_id_idx` (`order_id`)
);

CREATE TABLE IF NOT EXISTS `orders` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `user_id` varchar(36) NOT NULL,
  `status` ENUM ('COMPLETED', 'SHIPPING_ON_PROCESS', 'PAID', 'WAITING_PAYMENT') NOT NULL DEFAULT ('WAITING_PAYMENT'),
  `awb` varchar(255),
  `shipping_fee` int,
  `total_product_cost` int NOT NULL,
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  INDEX `orders_user_id_idx` (`user_id`),
  INDEX `orders_status_idx` (`status`),
  INDEX `orders_awb_idx` (`awb`),
  INDEX `orders_created_at_idx` (`created_at`),
  INDEX `orders_updated_at_idx` (`updated_at`)
);

CREATE TABLE IF NOT EXISTS `public_data` (
  `id` varchar(36) PRIMARY KEY DEFAULT (UUID()),
  `key` varchar(255) NOT NULL,
  `value` varchar(255),
  `created_at` timestamp DEFAULT (now()),
  `updated_at` timestamp DEFAULT (now()),
  UNIQUE INDEX `public_data_index_11` (`key`)
);
