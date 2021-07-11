export default function WithholdingCertificate() {
  return (
    <div className="w3-card-4">
      <div className="w3-container w3-green">
        <h2>Header</h2>
      </div>

      <form className="w3-container">
        <div>
          <label>First Name </label>
          <input className="w3-input" name="first_name" />
        </div>
        <div>
          <label>Middle Name </label>
          <input className="w3-input" name="middle_name" />
        </div>
        <div>
          <label>Last Name </label>
          <input className="w3-input" name="last_name" />
        </div>
        <div>
          <label>Street Address </label>
          <input className="w3-input" name="street_address" />
        </div>
        <div>
          <label>Mailing City </label>
          <input className="w3-input" name="mailing_city" />
        </div>
        <div>
          <label>Mailing State </label>
          <input className="w3-input" name="mailing_state" />
        </div>
        <div>
          <label>Mailing Zipcode </label>
          <input className="w3-input" name="mailing_zipcode" />
        </div>
        <div>
          <label htmlFor="filing status select">Filing Status: </label>
          <select name="filing_status" id="filing status select">
            <option value="single">Single</option>
            <option value="separate">Married Filing Separatly</option>
            <option value="joint">Married Filing Jointly</option>
            <option value="widow">Qualified Widow(er)</option>
            <option value="head_of_household">Head of Household</option>
          </select>
        </div>

        <div>
          <label>Have more than one Job </label>
          <input type="checkbox" className="w3-input" name="multiple_jobs" />
        </div>
        <div>
          <label>Married filing jointly and spouse also works</label>
          <input
            type="checkbox"
            className="w3-input"
            name="spousal_employment"
          />
        </div>
        <div>
          <label>Additional Income (not from jobs):</label>
          <input className="w3-input" name="other_income" />
        </div>
        <div>
          <label>Deductions other than Standard Deductions</label>
          <input className="w3-input" name="itemized_deduction_estimated" />
        </div>
        <div>
          <label>Extra Amount to withhold each pay period:</label>
          <input className="w3-input" name="extra_withholding" />
        </div>
      </form>
    </div>
  );
}
