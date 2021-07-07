import json
from datetime import date

import pytest
from django.contrib.auth import get_user_model
from django.test import TestCase


def get_user(_username):
    return get_user_model().objects.get(username=_username)


class TestProfile(TestCase):
    fixtures = ("init.json",)
    pytestmark = pytest.mark.django_db

    @classmethod
    def setUpClass(cls):
        super().setUpClass()
        cls.users = get_user_model().objects.all()
        # cls.address = Address.objects.get(idempotent_key="ckpfzqd7l0000nbve3vq1hfgl")

    @classmethod
    def tearDownClass(cls):
        super().tearDownClass()

    def test_users_loaded(self):
        assert len(self.users) > 0
