interface ComputerBuilder {
	default boolean isGraphicsCardEnabled() {
		return false;
	}
	default boolean isBluetoothEnabled() {
		return false;
	}
	String HDD();
	String RAM();
  //
	static Computer smallVM() {
			return new Computer(
					new ComputerBuilder() {
						public String HDD() { return "100 GB"; }
						public String RAM() { return "1 GB"; }
					}
			);
	}
	static Computer mediumVM() {
			return new Computer(
					new ComputerBuilder() {
						public boolean isGraphicsCardEnabled() { return true; }
						public String HDD() { return "500 GB"; }
						public String RAM() { return "4 GB"; }
					}
			);
	}
	static Computer largeVM() {
			return new Computer(
					new ComputerBuilder() {
							public boolean isGraphicsCardEnabled() { return true; }
							public boolean isBluetoothEnabled() { return true; }
							public String HDD() { return "1000 GB"; }
							public String RAM() { return "16 GB"; }
					}
			);
	}
}

class Computer {
	private String HDD;
	private String RAM;
	private boolean isGraphicsCardEnabled;
	private boolean isBluetoothEnabled;
  //
	public String getHDD() {
		return HDD;
	}
	public String getRAM() {
		return RAM;
	}
	public boolean isGraphicsCardEnabled() {
		return isGraphicsCardEnabled;
	}
	public boolean isBluetoothEnabled() {
		return isBluetoothEnabled;
	}
	public Computer(ComputerBuilder builder) {
		this.HDD=builder.HDD();
		this.RAM=builder.RAM();
		this.isGraphicsCardEnabled=builder.isGraphicsCardEnabled();
		this.isBluetoothEnabled=builder.isBluetoothEnabled();
	}
}
	
public class TestBuilderPattern {
	public static void main(String[] args) {
		Computer smallVM = ComputerBuilder.smallVM();
		Computer mediumVM = ComputerBuilder.mediumVM();
		Computer largeVM = ComputerBuilder.largeVM();
		Computer customVM = new Computer( 
				new ComputerBuilder() {
						public boolean isGraphicsCardEnabled() { return true; }
						public boolean isBluetoothEnabled() { return true; }
						public String HDD() { return "500 GB"; }
						public String RAM() { return "1 GB"; }
				}
		);
	}
}


