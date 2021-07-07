from django.db.models import (
    CASCADE,
    BooleanField,
    CharField,
    CheckConstraint,
    DateField,
    DateTimeField,
    ManyToManyField,
    Model,
    OneToOneField,
    TextChoices,
    FileField,
    TextField,
)
from django.contrib.auth import get_user_model
class QualifyingChild(Model):
    user = OneToOneField(User, related_name="owner", on_delete=CASCADE)

    def  passes_earned_income_tax_credit_disability(self):
        permanently_and_totally_disabled  = None
    def passes_earned_income_tax_credit_age(self):
        ...
    def passes_earned_income_tax_credit_relationship(self,adopted=False):
        # certificate_of_adoption
        ...
    def passes_earned_income_tax_credit_residency(self):
        ...
    def passes_earned_income_tax_credit_joint_return(self):
        ...