from datetime import date, timedelta
from django.contrib.auth import get_user_model
from cuid import cuid
from django.conf import settings
from django.contrib.auth.models import AbstractUser
from django.core.exceptions import ValidationError
from django.core.mail import send_mail
from django.db.models import (
    CASCADE,
    BooleanField,
    CharField,
    CheckConstraint,
    DateField,
    DateTimeField,
    ImageField,
    ManyToManyField,
    Model,
    OneToOneField,
    Q,
    TextChoices,
    TextField,
)
from django.utils.translation import gettext_lazy as _
from addresses.models import Address
User = settings.AUTH_USER_MODEL
# Testing ideas https://developer.mozilla.org/en-US/docs/Learn/Server-side/Django/Testing

class User(AbstractUser):
    first_name = CharField(max_length=100, blank=True, null=True)
    middle_name = CharField(max_length=20, blank=True, null=True)
    last_name = CharField(max_length=100, blank=True, null=True)
    madien_name = CharField(max_length=100, blank=True, null=True)
    suffix = CharField(max_length=10, blank=True, null=True)
    # internal_notes = TextField(default="", null=True, blank=True)
    # def clean_fields(self):
    #     super().clean_fields()
    @property
    def full_name(self):
        self.normalizer()
        return self.get_legal_name()
    def normalizer(self):      
        if self.first_name:
            self.first_name.strip().capitalize() 
        if self.middle_name:
            self.middle_name.strip().capitalize() 
        if self.last_name:
            self.last_name.strip().capitalize() 
        if self.madien_name:
            self.madien_name.strip().capitalize() 
        if self.suffix:
            self.suffix.strip().capitalize()
        self.save()
    def get_legal_name(self):
        full_name = ""
        full_name += f"{self.first_name}" if self.first_name else ""
        full_name += f" {self.middle_name}" if self.middle_name else ""
        full_name += f" {self.last_name}" if self.last_name else ""
        full_name += f" {self.suffix}" if self.suffix else ""
        return full_name

    def __str__(self):
        return f"User account: {self.full_name}"             



def user_directory_path(instance, filename):
    return f"user_{instance.user.id}/profile_pics/{filename}"


class Profile(Model):
    class Meta:
        constraints = [
            CheckConstraint(
                check=Q(date_of_death__lte=date.today() + timedelta(days=1)),
                name="not_dead_tomorrow",
            ),
            CheckConstraint(
                check=Q(date_of_birth__lte=date.today()), name="born_before_today"
            ),
        ]

    user = OneToOneField(User, on_delete=CASCADE)
    date_created = DateTimeField(auto_now_add=True, null=True, blank=True)
    date_modified = DateTimeField(auto_now=True, null=True, blank=True)
    date_of_birth = DateField(blank=True, null=True)
    date_of_death = DateField(null=True, blank=True)
    addresses = ManyToManyField(Address, blank=True)
    retention_only = BooleanField(default=False)
    do_not_contact = BooleanField(default=False)    
    internal_notes = TextField(default="", null=True, blank=True)

    def __str__(self):
        return f" {self.user.full_name}"
    @property
    def mark_decedent_inactive(self):
        self.is_active = False

    @property
    def mark_retention_only(self):
        self.retention_only = True

    def handle_deceased(self):
        if self.date_of_death is not None and date.today() > self.date_of_death:
            return ("Cmpl",self.mark_retention_only, self.mark_decedent_inactive)

