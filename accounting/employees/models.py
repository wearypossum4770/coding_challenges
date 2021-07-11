from uuid import uuid4
from django.utils.translation import gettext_lazy as _

from django.contrib.auth.models import AbstractUser
from django.core.validators import MaxValueValidator, MinValueValidator
from django.db.models import (CharField, DateField, DateTimeField,
                              DecimalField, EmailField, ForeignKey,BooleanField,
                              IntegerChoices, IntegerField, ManyToManyField,CASCADE,
                              Model, UUIDField)


# Create your models here.
class Salary(Model):
    taxable_wages = DecimalField(decimal_places=2, max_digits=11)

class Position(Model):
    class Grade(IntegerChoices):
        captain = 8055, _("captain")
        lieutenant = 7611, _("lieutenant")
        seat_1 = 7309, _("seat_1")
        seat_2 = 7131, _("seat_2")
        seat_3 = 6714, _("seat_3")
        seat_4 = 6419, _("seat_4")
        seat_5 = 6070, _("seat_5")
        seat_6 = 5890, _("seat_6")
        seat_7 = 5552, _("seat_7")
        seat_8 = 5540, _("seat_8")
        seat_9 = 5241, _("seat_9")
        seat_10 = 5136, _("seat_10")
        seat_11 = 4946, _("seat_11")
        seat_12 = 4172, _("seat_12")
        seat_13 = 4136, _("seat_13")
        seat_14 = 3502, _("seat_14")
        seat_15 = 2747, _("seat_15")
        seat_16 = 2303, _("seat_16")
        seat_17 = 1943, _("seat_17")
        seat_18 = 1733, _("seat_18")
    pay_rate = IntegerField(
        choices=Grade.choices,
        default=Grade.seat_18,
        validators=[
            MaxValueValidator(limit_value=8055),
            MinValueValidator(limit_value=1733),
        ]
    )
    is_current = BooleanField(default=True)
    start_date = DateField(null=True, blank=True)
    end_date = DateField(null=True, blank=True)


table_3 = {
    "weekly": 52,
    "bi_weekly": 26,
    "semi_monthly": 24,
    "monthly": 12,
    "quarterly": 4,
    "semi_annually": 2,
    "annually": 1,
    "daily": 260,
    "miscellaneous": 260,
}
class Department(Model):
    name = CharField(max_length=100, null=True, blank=True)

class Employee(Model):
    """
    Maximum pay_rate in millions per year
    """
    # employeeID = UUIDField(default=uuid4, null=True, blank=True)
    first_name = CharField(max_length=100, null=True, blank=True)
    middle_name = CharField(max_length=100, null=True, blank=True)
    last_name = CharField(max_length=100, null=True, blank=True)
    date_of_birth = DateField(null=True, blank=True)
    email = EmailField(null=True, blank=True)
    phone_number = CharField(max_length=15, null=True, blank=True)
    department = ForeignKey(Department, on_delete=CASCADE)
    clock_id = CharField(max_length=15,null=True, blank=True)
    # salaries_and_wages = ManyToManyField(Salary)
    position = ManyToManyField(Position)
    start_date = DateField(null=True, blank=True)
    end_date = DateField(null=True, blank=True)
    date_created = DateTimeField(auto_now_add=True, null=True, blank=True)
    date_modified = DateTimeField(auto_now=True, null=True, blank=True)
    def current_position(self):
        self.position
    def pay_rate_per_unit(self, total=None, unit=None):
        return total / table_3.get(unit)


    def begin_employment(self):
        return

    def end_employment(self):
        return

    def time_in_service(self):
        return

    def time_in_grade(self):
        return


