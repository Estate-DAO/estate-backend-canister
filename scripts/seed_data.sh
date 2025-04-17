# Test Case 1: Successful booking with paid status
dfx canister call estate_backend add_booking '(
  "alice@example.com",
  record {
    booking_id = record {
      app_reference = "BOOK_SUCCESS_001";
      email = "alice@example.com"
    };
    guests = record {
      adults = vec {
        record {
          first_name = "Alice";
          last_name = opt "Smith";
          email = opt "alice@example.com";
          phone = opt "+1-555-0123"
        }
      };
      children = vec {
        record {
          age = 8;
          first_name = "Bob";
          last_name = opt "Smith"
        }
      }
    };
    book_room_status = opt record {
      status = "CONFIRMED";
      message = "Booking confirmed successfully";
      commit_booking = record {
        api_status = variant { Confirmed };
        booking_ref_no = "REF123";
        booking_status = "CONFIRMED";
        confirmation_no = "CNF123";
        booking_id = record {
          app_reference = "BOOK_SUCCESS_001";
          email = "alice@example.com"
        };
        travelomatrix_id = "TM123";
        resolved_booking_status = variant { BookingConfirmed }
      }
    };
    user_selected_hotel_room_details = record {
      hotel_details = record {
        hotel_code = "HILTON123";
        hotel_name = "Hilton Garden Inn";
        hotel_image = "https://example.com/hilton.jpg";
        block_room_id = "BLOCK789";
        hotel_location = "Downtown, New York";
        hotel_token = "TOKEN789"
      };
      room_details = vec {
        record {
          room_price = 299.99;
          room_unique_id = "DLXROOM123";
          room_type_name = "Deluxe King"
        }
      };
      destination = opt record {
        city_id = "NYC";
        city = "New York";
        country_code = "US";
        country_name = "United States"
      };
      requested_payment_amount = 299.99;
      date_range = record {
        start = record { 2025; 3; 24 };
        end = record { 2025; 3; 26 }
      }
    };
    payment_details = record {
      booking_id = record {
        app_reference = "BOOK_SUCCESS_001";
        email = "alice@example.com"
      };
      payment_status = variant { Paid = "COMPLETED" };
      payment_api_response = record {
        updated_at = "2025-03-24T19:05:40";
        actually_paid = 299.99;
        provider = "STRIPE";
        invoice_id = 98765;
        order_description = "2 nights at Hilton Garden Inn";
        pay_amount = 299.99;
        pay_currency = "USD";
        created_at = "2025-03-24T19:05:40";
        payment_status = "COMPLETED";
        price_amount = 299;
        purchase_id = 98765;
        order_id = "ORDER_SUCCESS_001";
        price_currency = "USD";
        payment_id = 98765
      }
    }
  }
)'

# Test Case 2: Booking with unpaid status
dfx canister call estate_backend add_booking '(
  "bob@example.com",
  record {
    booking_id = record {
      app_reference = "BOOK_UNPAID_001";
      email = "bob@example.com"
    };
    guests = record {
      adults = vec {
        record {
          first_name = "Bob";
          last_name = opt "Johnson";
          email = opt "bob@example.com";
          phone = opt "+1-555-4567"
        }
      };
      children = vec {}
    };
    book_room_status = null;
    user_selected_hotel_room_details = record {
      hotel_details = record {
        hotel_code = "MARRIOTT456";
        hotel_name = "Marriott Downtown";
        hotel_image = "https://example.com/marriott.jpg";
        block_room_id = "BLOCK456";
        hotel_location = "Financial District, San Francisco";
        hotel_token = "TOKEN456"
      };
      room_details = vec {
        record {
          room_price = 399.99;
          room_unique_id = "SUITE456";
          room_type_name = "Executive Suite"
        }
      };
      destination = opt record {
        city_id = "SFO";
        city = "San Francisco";
        country_code = "US";
        country_name = "United States"
      };
      requested_payment_amount = 399.99;
      date_range = record {
        start = record { 2025; 3; 24 };
        end = record { 2025; 3; 25 }
      }
    };
    payment_details = record {
      booking_id = record {
        app_reference = "BOOK_UNPAID_001";
        email = "bob@example.com"
      };
      payment_status = variant { Unpaid = opt "PAYMENT_FAILED" };
      payment_api_response = record {
        updated_at = "2025-03-24T19:05:40";
        actually_paid = 0.00;
        provider = "STRIPE";
        invoice_id = 54321;
        order_description = "1 night at Marriott Downtown";
        pay_amount = 399.99;
        pay_currency = "USD";
        created_at = "2025-03-24T19:05:40";
        payment_status = "FAILED";
        price_amount = 399;
        purchase_id = 54321;
        order_id = "ORDER_FAILED_001";
        price_currency = "USD";
        payment_id = 54321
      }
    }
  }
)'


# Test Case 3: no payment status + booking not confirmed 
dfx canister call estate_backend add_booking '(
  "alice1@example.com",
  record {
    booking_id = record {
      app_reference = "HB2403-15938-78748";
      email = "alice1@example.com"
    };
    guests = record {
      adults = vec {
        record {
          first_name = "Alice1";
          last_name = opt "Smith1";
          email = opt "alice1@example.com";
          phone = opt "+1-555-0123"
        }
      };
      children = vec {
        record {
          age = 8;
          first_name = "Bob1";
          last_name = opt "Smith1"
        }
      }
    };
    book_room_status = null;
    user_selected_hotel_room_details = record {
      hotel_details = record {
        hotel_code = "HILTON123";
        hotel_name = "Hilton Garden Inn";
        hotel_image = "https://example.com/hilton.jpg";
        block_room_id = "BLOCK789";
        hotel_location = "Downtown, New York";
        hotel_token = "TOKEN789"
      };
      room_details = vec {
        record {
          room_price = 999.99;
          room_unique_id = "DLXROOM123";
          room_type_name = "Deluxe King"
        }
      };
      destination = opt record {
        city_id = "NYC";
        city = "New York";
        country_code = "US";
        country_name = "United States"
      };
      requested_payment_amount = 999.99;
      date_range = record {
        start = record { 2025; 3; 24 };
        end = record { 2025; 3; 26 }
      }
    };
    payment_details = record {
      booking_id = record {
        app_reference = "HB2403-15938-78748";
        email = "alice1@example.com"
      };
      payment_status = variant { Unpaid = opt "" };
      payment_api_response = record {
        updated_at = "";
        actually_paid = 0.00;
        provider = "";
        invoice_id = 0  ;
        order_description = "";
        pay_amount = 0.00;
        pay_currency = "";
        created_at = "";
        payment_status = "";
        price_amount = 0;
        purchase_id = 0;
        order_id = "";
        price_currency = "";
        payment_id = 0
      }
    }
  }
)'


# Test Case 4: Booking with pending status
dfx canister call estate_backend add_booking '(
  "user@example.com",
  record {
    booking_id = record {
      app_reference = "ABC123";
      email = "user@example.com"
    };
    guests = record {
      adults = vec {
        record {
          first_name = "John";
          last_name = opt "Doe";
          email = opt "user@example.com";
          phone = opt "+1-555-9876"
        }
      };
      children = vec {}
    };
    book_room_status = opt record {
      status = "PENDING";
      message = "Booking is being processed";
      commit_booking = record {
        api_status = variant { BookFailed };
        booking_ref_no = "";
        booking_status = "";
        confirmation_no = "";
        booking_id = record {
          app_reference = "ABC123";
          email = "user@example.com"
        };
        travelomatrix_id = "TM456";
        resolved_booking_status = variant { Unknown }
      }
    };
    user_selected_hotel_room_details = record {
      hotel_details = record {
        hotel_code = "HYATT789";
        hotel_name = "Grand Hyatt";
        hotel_image = "https://example.com/hyatt.jpg";
        block_room_id = "BLOCK123";
        hotel_location = "Downtown, Chicago";
        hotel_token = "TOKEN123"
      };
      room_details = vec {
        record {
          room_price = 499.99;
          room_unique_id = "PREM789";
          room_type_name = "Premium Suite"
        }
      };
      destination = opt record {
        city_id = "CHI";
        city = "Chicago";
        country_code = "US";
        country_name = "United States"
      };
      requested_payment_amount = 499.99;
      date_range = record {
        start = record { 2025; 4; 15 };
        end = record { 2025; 4; 17 }
      }
    };
    payment_details = record {
      booking_id = record {
        app_reference = "ABC123";
        email = "user@example.com"
      };
      payment_status = variant { Unpaid = opt "" };
      payment_api_response = record {
        updated_at = "";
        actually_paid = 0.00;
        provider = "";
        invoice_id = 0  ;
        order_description = "";
        pay_amount = 0.00;
        pay_currency = "";
        created_at = "";
        payment_status = "";
        price_amount = 0;
        purchase_id = 0;
        order_id = "";
        price_currency = "";
        payment_id = 0
      }
    }
  }
)'
