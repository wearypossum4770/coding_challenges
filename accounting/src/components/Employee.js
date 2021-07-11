export default class Employee {
  constructor({
    employee_id,
    first_name,
    middle_name,
    last_name,
    date_of_birth,
    email,
    phone_number,
    department,
    position,
    start_date,
    end_date,
    date_created,
    date_modified,
  }) {
    this.employee_id = employee_id;
    this.firstName = first_name;
    this.middleName = middle_name;
    this.lastName = last_name;
    this.dateOfBirth = date_of_birth;
    this.email = email;
    this.phoneNumber = phone_number;
    this._department = department;
    this.position = position;
    this.startDate = start_date;
    this.endDate = end_date;
    this.dateCreated = date_created;
    this.dateModified = date_modified;
  }
}
